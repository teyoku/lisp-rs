use std::rc::Rc;

use crate::{
    ast::{Expr, ExprKind},
    callable::Lambda,
    environment::{Environment, EnvironmentRef},
    error::{LispError, LispErrorKind},
    value::Value,
};

#[derive(Default)]
pub struct Evaluator {
    global: EnvironmentRef,
}

impl Evaluator {
    pub fn with_environment(global: EnvironmentRef) -> Self {
        Self { global }
    }

    pub fn global_environment(&self) -> EnvironmentRef {
        self.global.clone()
    }

    pub fn eval_program(&mut self, expressions: &[Expr]) -> Result<Value, LispError> {
        if expressions.is_empty() {
            return Ok(Value::Nil);
        }

        let mut result = Value::Nil;
        let global = self.global.clone();

        for expr in expressions {
            result = self.eval(expr, &global)?;
        }

        Ok(result)
    }

    pub fn eval(
        &mut self,
        expression: &Expr,
        environment: &EnvironmentRef,
    ) -> Result<Value, LispError> {
        match &expression.kind {
            ExprKind::Integer(value) => Ok(Value::Integer(*value)),
            ExprKind::Float(value) => Ok(Value::Float(*value)),
            ExprKind::Boolean(value) => Ok(Value::Boolean(*value)),
            ExprKind::String(value) => Ok(Value::String(Rc::from(value.as_str()))),
            ExprKind::Symbol(name) => Environment::get(environment, name),
            ExprKind::List(exprs) => {
                if exprs.is_empty() {
                    return Ok(Value::Nil);
                }

                self.eval_list(exprs, environment)
            }
            ExprKind::Quote(expr) => Ok(self.quote_expr(expr)),
        }
    }

    fn eval_list(
        &mut self,
        items: &[Expr],
        environment: &EnvironmentRef,
    ) -> Result<Value, LispError> {
        let first = &items[0];

        if let ExprKind::Symbol(name) = &first.kind {
            match name.as_str() {
                "define" => {
                    if items.len() != 3 {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm("define".to_string()),
                            first.span,
                        ));
                    }

                    if let ExprKind::Symbol(var_name) = &items[1].kind {
                        let value = self.eval(&items[2], environment)?;
                        Environment::define(environment, var_name, value);

                        return Ok(Value::Nil);
                    } else {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm(
                                "expected symbol after define".to_string(),
                            ),
                            items[1].span,
                        ));
                    }
                }
                "set!" => {
                    if items.len() != 3 {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm("set!".to_string()),
                            first.span,
                        ));
                    }
                    if let ExprKind::Symbol(var_name) = &items[1].kind {
                        let value = self.eval(&items[2], environment)?;
                        Environment::assign(environment, var_name, value)?;
                        return Ok(Value::Nil);
                    } else {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm(
                                "expected symbol after set!".to_string(),
                            ),
                            items[1].span,
                        ));
                    }
                }
                "if" => {
                    if items.len() != 3 && items.len() != 4 {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm("if".to_string()),
                            first.span,
                        ));
                    }
                    let condition = self.eval(&items[1], environment)?;
                    if condition.is_truthy() {
                        return self.eval(&items[2], environment);
                    } else if items.len() == 4 {
                        return self.eval(&items[3], environment);
                    } else {
                        return Ok(Value::Nil);
                    }
                }
                "lambda" => {
                    if items.len() < 3 {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm("lambda".to_string()),
                            first.span,
                        ));
                    }
                    let mut parameters = Vec::new();
                    if let ExprKind::List(params) = &items[1].kind {
                        for param in params {
                            if let ExprKind::Symbol(param_name) = &param.kind {
                                parameters.push(param_name.clone());
                            } else {
                                return Err(LispError::at(
                                    LispErrorKind::InvalidSpecialForm(
                                        "lambda parameters must be symbols".to_string(),
                                    ),
                                    param.span,
                                ));
                            }
                        }
                    } else {
                        return Err(LispError::at(
                            LispErrorKind::InvalidSpecialForm(
                                "expected parameter list".to_string(),
                            ),
                            items[1].span,
                        ));
                    }

                    let body = items[2..].to_vec();
                    let lambda = Lambda {
                        parameters,
                        body,
                        closure: environment.clone(),
                    };
                    return Ok(Value::Lambda(Rc::new(lambda)));
                }
                _ => {}
            }
        }

        let callable = self.eval(first, environment)?;
        let arguments = self.eval_arguments(&items[1..], environment)?;
        self.apply(callable, arguments)
    }

    fn eval_arguments(
        &mut self,
        arguments: &[Expr],
        environment: &EnvironmentRef,
    ) -> Result<Vec<Value>, LispError> {
        let mut evaluated = Vec::with_capacity(arguments.len());
        for arg in arguments {
            evaluated.push(self.eval(arg, environment)?);
        }
        Ok(evaluated)
    }

    fn apply(&mut self, callable: Value, arguments: Vec<Value>) -> Result<Value, LispError> {
        match callable {
            Value::BuiltinFunction(builtin) => builtin.call(&arguments),
            Value::Lambda(lambda) => self.apply_lambda(&lambda, arguments),
            _ => Err(LispError::new(LispErrorKind::NotCallable(
                callable.type_name().to_string(),
            ))),
        }
    }

    fn apply_lambda(
        &mut self,
        lambda: &Lambda,
        arguments: Vec<Value>,
    ) -> Result<Value, LispError> {
        if lambda.parameters.len() != arguments.len() {
            return Err(LispError::new(LispErrorKind::WrongArity {
                function: "lambda".to_string(),
                expected: lambda.parameters.len().to_string(),
                received: arguments.len(),
            }));
        }

        let local_env = Environment::enclosed(&lambda.closure);

        for (param_name, arg_value) in lambda.parameters.iter().zip(arguments) {
            Environment::define(&local_env, param_name, arg_value);
        }

        let mut result = Value::Nil;
        for expr in &lambda.body {
            result = self.eval(expr, &local_env)?;
        }

        Ok(result)
    }

    fn quote_expr(&self, expr: &Expr) -> Value {
        match &expr.kind {
            ExprKind::Integer(v) => Value::Integer(*v),
            ExprKind::Float(v) => Value::Float(*v),
            ExprKind::Boolean(v) => Value::Boolean(*v),
            ExprKind::String(v) => Value::String(Rc::from(v.as_str())),
            ExprKind::Symbol(v) => Value::Symbol(Rc::from(v.as_str())),
            ExprKind::List(exprs) => {
                let values: Vec<Value> = exprs.iter().map(|e| self.quote_expr(e)).collect();
                Value::List(Rc::new(values))
            }
            ExprKind::Quote(inner) => {
                let inner_val = self.quote_expr(inner);
                let quote_sym = Value::Symbol(Rc::from("quote"));
                Value::List(Rc::new(vec![quote_sym, inner_val]))
            }
        }
    }
}
