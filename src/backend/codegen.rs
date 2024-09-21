use core::panic;

use inkwell::{types::{AnyType, AnyTypeEnum, BasicType}, values::{AnyValue, AnyValueEnum, BasicValue}};

use crate::frontend::{AstExpr, AstStatement, AstType, BinOpKind};

use super::{codegen_ctx::{CodegenContext, SymbolValue}, codegen_err::CodegenError};

type IRResult<T> = Result<T, CodegenError>;

pub type IRValueResult<'ir> = IRResult<AnyValueEnum<'ir>>;

pub type IRTypeResult<'ir> = IRResult<AnyTypeEnum<'ir>>;

pub trait Codegen<'ctx, 'ir> 
where 'ctx: 'ir
{
     fn codegen(&self, context: &CodegenContext<'ctx>) -> IRValueResult<'ir>;
}

impl<'ctx, 'ir> Codegen<'ctx, 'ir> for AstExpr 
where 'ctx: 'ir
{
    fn codegen(&self, codegen_context: &CodegenContext<'ctx>) -> IRValueResult<'ir> {
        match self {
            AstExpr::Int { value, .. } => {
                let int_value = codegen_context.context.i64_type().const_int(*value as u64, false);
                Ok(int_value.as_any_value_enum())
            }
            AstExpr::Float { value, .. } => {
                let float_value = codegen_context.context.f64_type().const_float(*value);
                Ok(float_value.as_any_value_enum())
            }
            AstExpr::Ident { name, loc } => {
                // we have to see if the identifier is defined in the symbol table
                match codegen_context.symbol_table.borrow().get(name) {
                    Some(symbol_value) => {
                        // we'll return a load instruction
                        let load_instr = codegen_context
                            .builder
                            .build_load(symbol_value.pointee_ty, symbol_value.ptr_val, "load_val")
                            .expect("Error: An error occured while building a load instruction");
                        Ok(load_instr.as_any_value_enum())
                    }
                    None => Err(CodegenError::UndefinedSymbol(name.clone(), *loc))
                }
            }
            AstExpr::BinOp { lhs, rhs, kind, loc } => {
                let lhs_val = lhs.codegen(codegen_context)?;
                let rhs_val = rhs.codegen(codegen_context)?;
                match kind {
                    BinOpKind::Add => {
                        match (lhs_val, rhs_val) {
                            (AnyValueEnum::IntValue(lhs_int), AnyValueEnum::IntValue(rhs_int)) => {
                                let result = codegen_context.builder.build_int_add(lhs_int, rhs_int, "add");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building add instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            (AnyValueEnum::FloatValue(lhs_float), AnyValueEnum::FloatValue(rhs_float)) => {
                                let result = codegen_context.builder.build_float_add(lhs_float, rhs_float, "add");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building add instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            _ => Err(CodegenError::UnimplementedOp(*kind, *lhs.clone(), *rhs.clone(), *loc))
                        }
                    }
                    BinOpKind::Sub => {
                        match (lhs_val, rhs_val) {
                            (AnyValueEnum::IntValue(lhs_int), AnyValueEnum::IntValue(rhs_int)) => {
                                let result = codegen_context.builder.build_int_sub(lhs_int, rhs_int, "sub");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building sub instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            (AnyValueEnum::FloatValue(lhs_float), AnyValueEnum::FloatValue(rhs_float)) => {
                                let result = codegen_context.builder.build_float_sub(lhs_float, rhs_float, "sub");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building sub instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            _ => Err(CodegenError::UnimplementedOp(*kind, *lhs.clone(), *rhs.clone(), *loc))
                        }
                    }
                    BinOpKind::Mul => {
                        match (lhs_val, rhs_val) {
                            (AnyValueEnum::IntValue(lhs_int), AnyValueEnum::IntValue(rhs_int)) => {
                                let result = codegen_context.builder.build_int_mul(lhs_int, rhs_int, "mul");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building mul instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            (AnyValueEnum::FloatValue(lhs_float), AnyValueEnum::FloatValue(rhs_float)) => {
                                let result = codegen_context.builder.build_float_mul(lhs_float, rhs_float, "mul");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building mul instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            _ => Err(CodegenError::UnimplementedOp(*kind, *lhs.clone(), *rhs.clone(), *loc))
                        }
                    }
                    BinOpKind::Div => {
                        match (lhs_val, rhs_val) {
                            (AnyValueEnum::IntValue(lhs_int), AnyValueEnum::IntValue(rhs_int)) => {
                                let result = codegen_context.builder.build_int_signed_div(lhs_int, rhs_int, "div");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building div instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            (AnyValueEnum::FloatValue(lhs_float), AnyValueEnum::FloatValue(rhs_float)) => {
                                let result = codegen_context.builder.build_float_div(lhs_float, rhs_float, "div");
                                if let Ok(instr_val) = result {
                                    Ok(instr_val.as_any_value_enum())
                                } else {
                                    panic!("Error: An error occured while building div instruction at location {:?} {:?}", loc, result.err())
                                }
                            }
                            _ => Err(CodegenError::UnimplementedOp(*kind, *lhs.clone(), *rhs.clone(), *loc))
                        }
                    }
                    _ => Err(CodegenError::UnimplementedOp(*kind, *lhs.clone(), *rhs.clone(), *loc)),
                }
            }
            _ => unimplemented!("codegen for {:?} has not been implemented", self)
        }
    }
}

impl<'ctx, 'ir> Codegen<'ctx, 'ir> for AstStatement
where 'ctx: 'ir
{
    fn codegen(&self, codegen_context: &CodegenContext<'ctx>) -> IRValueResult<'ir> {
        match self {
            AstStatement::Expr(expr) => expr.codegen(codegen_context),
            AstStatement::ConstDef { name, ty, value, loc } => {
                let value = match value.codegen(codegen_context)? {
                    AnyValueEnum::IntValue(int_val) => int_val.as_basic_value_enum(),
                    AnyValueEnum::FloatValue(float_val) => float_val.as_basic_value_enum(),
                    _ => panic!("Error: An error occured while getting the value of the constant definition at location {:?}", loc)
                };
                let ty = match ty.codegen(codegen_context)? {
                    AnyTypeEnum::IntType(int_ty) => int_ty.as_basic_type_enum(),
                    AnyTypeEnum::FloatType(float_ty) => float_ty.as_basic_type_enum(),
                    _ => panic!("Error: An error occured while getting the type of the constant definition at location {:?}", loc)
                };
                let value_ptr = codegen_context.builder.build_alloca(ty, name.as_str()).unwrap();
                let store_instr = codegen_context.builder.build_store(value_ptr, value).unwrap();
                let symbol_value = SymbolValue::constant(value_ptr, ty);
                codegen_context.symbol_table.borrow_mut().insert(name.clone(), symbol_value);
                Ok(store_instr.as_any_value_enum())
            }
            _ => unimplemented!("codegen for {:?} has not been implemented", self)
        }
    }
}

impl<'ctx, 'ir> AstType
where 'ctx: 'ir
{
    fn codegen(&self, codegen_context: &CodegenContext<'ctx>) -> IRTypeResult<'ir> {
        match self {
            AstType::Int => Ok(codegen_context.context.i64_type().as_any_type_enum()),
            AstType::Float => Ok(codegen_context.context.f64_type().as_any_type_enum()),
            AstType::UnsignedInt => Ok(codegen_context.context.i64_type().as_any_type_enum()),
            _ => unimplemented!("codegen for {:?} has not been implemented", self)
        }
    }
}