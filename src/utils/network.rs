use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use crate::global::global::{NO_PARAMS,CELO_PRICE};
use crate::utils::errors::MyError;

pub async fn get(vector0: Vec<String>, vector1: Vec<String>, url: String) -> Result<HashMap<String, String>, MyError> {
     if  vector0.len() == 0|| vector1.len()==0{
         return  Err(MyError::NotFound(NO_PARAMS.into()))
     }
    // post 请求要创建client
    let client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    // headers.insert("Content-Type", mytype);
    let type_value = HeaderValue::from_str("application/json").unwrap();
    headers.insert("Content-Type", type_value);
//     // 组装要提交的数据
    let mut data = HashMap::new();
    for (index, value) in vector0.iter().enumerate() {
        vector1.get(index).map(|v| {
            let vo = vector0.get(index).unwrap();
            data.insert(vo.to_string(), v.to_string());
        });
    }
    //遍历打印data
    for (key, value) in &data {
        println!("{}:{}", key, value);
    }

    // 发起post请求并返回
    Ok(client
        .get(url)
        .headers(headers)
        .json(&data)
        .send().await?
        .json::<HashMap<String, String>>().await?)
    // Ok(reqwest::get("https://httpbin.org/ip").await?.json::<HashMap<String, String>>().await?)
}

pub async fn post(vector0: Vec<String>, vector1: Vec<String>, url: &String) -> Result<HashMap<String, Value>, MyError> {
    if  vector0.len() == 0|| vector1.len()==0{
        // return Err(MyError::InvalidTnput(NO_PARAMS.to_string()));
        return  Err(MyError::NotFound(NO_PARAMS.into()))
    }
    // post 请求要创建client
    let client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    // headers.insert("Content-Type", mytype);
    let type_value = HeaderValue::from_str("application/json").unwrap();
    headers.insert("Content-Type", type_value);
//     // 组装要提交的数据
    let mut data = HashMap::new();
    for (index, value) in vector0.iter().enumerate() {
        vector1.get(index).map(|v| {
            let vo = vector0.get(index).unwrap();
            data.insert(vo.to_string(), v.to_string());
        });
    }
    //遍历打印data
    for (key, value) in &data {
        println!("{}:{}", key, value);
    }

    // 发起post请求并返回
    Ok(client
        .post(url)
        .headers(headers)
        .json(&data)
        .send().await?
        .json::<HashMap<String, Value>>().await?)
}


pub async fn block_post(url: &String) -> Result<HashMap<String, Value>, MyError> {
    // post 请求要创建client
    let client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    // NewReader::new(`"{"query":"{ pair(id:\"0xb460f9ae1fea4f77107146c1960bb1c978118816\") {    token1Price}}"}"`).await
    // headers.insert("Content-Type", mytype);
    let mut data = HashMap::new();

    let type_value = (CELO_PRICE).to_string();
    data.insert("query",type_value);
    let type_value = HeaderValue::from_str("application/json").unwrap();
    headers.insert("Content-Type", type_value);
//     // 组装要提交的数据
    // 发起post请求并返回
    Ok(client
        .post(url)
        .headers(headers)
        .json(&data)
        .send().await?
        .json::<HashMap<String, Value>>().await?)
}


//test
#[test]
fn test_get() {
    //定义vector0 vector1
    let mut vector0 = Vec::new();
    let mut vector1 = Vec::new();
    //添加元素 1234 到vector0
    vector0.push("name".to_string());
    vector0.push("age".to_string());
    vector0.push("school".to_string());
    //添加元素 5678 到vector1
    vector1.push("mason".to_string());
    vector1.push("11".to_string());
    vector1.push("河南大学".to_string());

    let mut data = HashMap::new();
    for (index, value) in vector0.iter().enumerate() {
        vector1.get(index).map(|v| {
            let vo = vector0.get(index).unwrap();
            data.insert(vo.to_string(), v.to_string());
        });
    }
    //遍历打印data
    for (key, value) in data {
        println!("{}:{}", key, value);
    }
}
