use graphql_client::QueryBody;
use serde_json::{Value, json};

// Problematic code
// temporary value dropped while borrowed
// fn build_top_pools_query(field: &str, num_pools: i32) -> QueryBody<Value> {
//     let query = format!(
//         r#"query topPools($numPools: Int!) {{
//   {field}(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {{
//     id
//     volumeUSD
//     token0 {{
//       id
//       name
//       decimals
//     }}
//     token1 {{
//       id
//       name
//       decimals
//     }}
//   }}
// }}"#
//     )
//     .as_str();
//
//     QueryBody {
//         variables: json!({"numPools": num_pools}),
//         query,
//         operation_name: "topPools",
//     }
// }
// fn build_top_pools_query(field: &str, num_pools: i32) -> QueryBody<Value> {
//     let query = format!(
//         r#"query topPools($numPools: Int!) {{
//   {field}(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {{
//     id
//     volumeUSD
//     token0 {{
//       id
//       name
//       decimals
//     }}
//     token1 {{
//       id
//       name
//       decimals
//     }}
//   }}
// }}"#
//     );
//
//     QueryBody {
//         variables: json!({"numPools": num_pools}),
//         query: &query,
//         operation_name: "topPools",
//     }
// }

fn build_v2_top_pools_query(field: &str, num_pools: i32) -> QueryBody<Value> {
    let query = r#"
query topPools($numPools: Int!) {
  pairs(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {
    id
    volumeUSD
    token0 {
      id
      name
      decimals
    }
    token1 {
      id
      name
      decimals
    }
  }
}"#;

    QueryBody {
        variables: json!({"numPools": num_pools}),
        query,
        operation_name: "topPools",
    }
}

fn build_v3_top_pools_query(field: &str, num_pools: i32) -> QueryBody<Value> {
    let query = r#"
query topPools($numPools: Int!) {
  pools(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {
    id
    volumeUSD
    token0 {
      id
      name
      decimals
    }
    token1 {
      id
      name
      decimals
    }
  }
}"#;

    QueryBody {
        variables: json!({"numPools": num_pools}),
        query,
        operation_name: "topPools",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_top_pools_query() {
        let query = build_v2_top_pools_query("pools", 5);
        let query = build_v3_top_pools_query("pools", 5);
    }
}
