use sqlx::FromRow;
use sqlx_model::SqlxModel;

// model : detail_coin [  id  exchange_id  coin_name  coin_contract  pairs_id  query_command ]

#[derive(sqlx::FromRow,sqlx_model::SqlxModel,Clone,Debug)]
#[sqlx_model(table_name="detail_coin")]
pub struct detail_coin {

    #[sqlx(default)]
    #[sqlx(rename="id")]
    pub id: i32,

    /// 交易平台  default:  None
    #[sqlx(default)]
    #[sqlx(rename="exchange_id")]
    pub exchangeId: i32,

    /// 币种名  default:  None
    #[sqlx(default)]
    #[sqlx(rename="coin_name")]
    pub coinName: String,

    /// 合约名  default:  None
    #[sqlx(default)]
    #[sqlx(rename="coin_contract")]
    pub coinContract: String,

    /// 交易对名  default:  None
    #[sqlx(default)]
    #[sqlx(rename="pairs_id")]
    pub pairsId: String,


    #[sqlx(default)]
    #[sqlx(rename="query_command")]
    pub queryCommand: String,
}

