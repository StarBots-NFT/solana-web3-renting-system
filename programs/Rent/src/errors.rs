use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The item isn't active")]
  NotActiveItem,
  #[msg("The item isn't ended")]
  NotEndedItem,
  #[msg("wrong period time")]
  InvalidatePeriodTime,
  #[msg("wrong owner address")]
  WrongOwnerAddress,
  #[msg("wrong collection address")]
  WrongCollectionAddress,
}
