use num_traits::{PrimInt, CheckedAdd, Zero};
 
#[derive(Debug, PartialEq)]
pub enum DivisionError {
    DivisionByZero,
    Overflow,
}
 
pub fn divide<T>(dividend: T, divisor: T) -> Result<T, DivisionError>
where
    T: PrimInt + CheckedAdd + PartialEq + Zero + Copy,
{
    // Verifica se o divisor é zero
    // Escreva seu código aqui
    if divisor.is_zero() {
      return Err(DivisionError::DivisionByZero);
    }
 
    // Verifica se houve overflow
    // Escreva seu código aqui
    if divisor.checked_add(&divisor).is_none() {
      return Err(DivisionError::Overflow);
    }
 
    // Realiza a divisão
    // Escreva seu código aqui
    Ok(dividend / divisor)
}
 
#[test]
fn test_divide() {
    // Teste bem-sucedido de divisão
    assert_eq!(divide(10, 2), Ok(5));
 
    // Teste de divisão por zero
    // Escreva seu teste aqui
    assert_eq!(divide(5, 0), Err(DivisionError::DivisionByZero));
 
    // Teste de divisão que resulta em overflow
    // Escreva seu teste aqui
    assert_eq!(divide(255u8, 255u8), Err(DivisionError::Overflow));
}