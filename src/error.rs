use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]


   // Std(#[from] StdError) seçeneği, smart contract'ın çalışması sırasında standart 
   // hata türleriyle karşılaşılması durumunda bu hata türlerinin ContractError türüne dönüştürülmesi işlemini yapar.
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
   
}