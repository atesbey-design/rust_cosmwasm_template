
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


//InstantiateMsg: Bu tip, smart contract'in ilk kez başlatıldığında alacağı parametreleri tanımlar. 
//Bu örnekte, InstantiateMsg yalnızca count adlı bir tam sayı içermektedir.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
 
}



//ExecuteMsg: Bu tip, smart contract'ın yürütülebilir mesajlarını tanımlar. 
//Bu örnekte, ExecuteMsg, Increment ve Reset adlı iki alt mesaj tiplerini içermektedir. 
//Increment mesajı, mevcut sayacın değerini arttırmak için kullanılırken, Reset mesajı, sayacın belirtilen bir değere sıfırlanması için kullanılır.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}



//QueryMsg: Bu tip, smart contract'ın sorgulanabilir mesajlarını tanımlar. Bu örnekte, yalnızca GetCount adlı bir mesaj tipi bulunmaktadır. 
//Bu mesaj, smart contract'ın mevcut sayac değerini JSON olarak kodlanmış bir sayı olarak döndürür.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCount {},
}



//CountResponse<Custom>: Bu tip, smart contract'ın mevcut sayac değerini döndürmesi için kullanılır.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}