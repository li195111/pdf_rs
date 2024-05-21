use pdf_extract::extract_text;
use std::{fs::File, io::Write};
// use regex::Regex;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
// use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct CodeRate {
    code: String,
    name: String,
    pure_profit_rate: String,
    income_standard: String,
    profit_rate: String,
    cost_rate: String,
    net_profit_rate: String,
}

impl CodeRate {
    fn new(
        code: String,
        name: String,
        pure_profit_rate: String,
        income_standard: String,
        profit_rate: String,
        cost_rate: String,
        net_profit_rate: String,
    ) -> Self {
        CodeRate {
            code,
            name,
            pure_profit_rate,
            income_standard,
            profit_rate,
            cost_rate,
            net_profit_rate,
        }
    }

    fn from_vec(vec_str: &Vec<&str>) -> Self {
        let code = vec_str[0].to_string();
        let name = vec_str[1].to_string();
        let pure_profit_rate = vec_str[2].to_string();
        let income_standard = vec_str[3].to_string();
        let profit_rate = vec_str[4].to_string();
        let cost_rate = vec_str[5].to_string();
        let net_profit_rate = vec_str[6].to_string();
        CodeRate::new(
            code,
            name,
            pure_profit_rate,
            income_standard,
            profit_rate,
            cost_rate,
            net_profit_rate,
        )
    }

    // fn get_field_value(&self, field_name: &str) -> Option<String> {
    //     let field_map = self.to_map();
    //     field_map.get(field_name).cloned()
    // }

    // fn to_map(&self) -> HashMap<&str, String> {
    //     let mut map = HashMap::new();
    //     map.insert("code", self.code.clone());
    //     map.insert("name", self.name.clone());
    //     map.insert("pure_profit_rate", self.pure_profit_rate.to_string());
    //     map.insert("profit_rate", self.profit_rate.to_string());
    //     map.insert("cost_rate", self.cost_rate.to_string());
    //     map.insert("net_profit_rate", self.net_profit_rate.to_string());
    //     map
    // }
}

fn main() {
    let file_path: &str = "src/docs/112年度營利事業各業擴大書審純益率、所得額及同業利潤標準.pdf";
    let text = extract_text(file_path).unwrap();

    let new_page_title_start = "標準代號";
    let new_page_title_end = "標準";
    let mut skip = false;
    let mut rows = Vec::<Vec<&str>>::new();
    let mut str_rows = Vec::<String>::new();
    let mut code_rate_datas = Vec::<CodeRate>::new();

    for line in text.lines() {
        let row_data = line.split(" ").collect::<Vec<_>>();
        if line.contains(new_page_title_start) {
            skip = true;
        }
        if !skip && row_data.len() >= 6 {
            let mut filted_row = row_data
                .into_iter()
                .filter(|&x| !x.trim().is_empty())
                .collect::<Vec<_>>();
            let last_val = *filted_row.last().unwrap();
            if filted_row[0].len() > 8 {
                let (first_ele, second_ele) = filted_row[0].split_at(8);
                filted_row.remove(0);
                let mut re_row_data = vec![first_ele, second_ele];
                let remain = filted_row.to_vec();
                re_row_data.extend(remain);
                // println!("{} {:?}", re_row_data.len(), re_row_data);
                filted_row.insert(0, second_ele);
                filted_row.insert(0, first_ele);
                // println!("{} {:?}", filted_row.len(), filted_row);
            } else if last_val.len() > 3 {
                // Not a number
                let last_ele = &last_val[..2];
                let name_ele = &last_val[2..];
                filted_row.remove(filted_row.len() - 1);
                filted_row.insert(1, name_ele);
                filted_row.push(last_ele);
            } else if filted_row[0].len() == 8 && filted_row.len() < 7 {
                filted_row.insert(1, "Unknow");
                // println!("{} {:?}", filted_row.len(), filted_row);
            } else {
                // println!("{} {:?}", filted_row.len(), filted_row);
            }
            let code_rate = CodeRate::from_vec(&filted_row);
            code_rate_datas.push(code_rate);

            let row_str = filted_row.join("|");
            rows.push(filted_row);
            str_rows.push(row_str);
        }
        if line == new_page_title_end {
            // println!("Page Title End !");
            skip = false;
        }
    }
    let str_cols: Vec<&str> = vec![
        "標準代號",
        "小業別",
        "擴大書審純益率",
        "所得額標準",
        "同業利潤標準毛利率",
        "同業利潤標準費用率",
        "同業利潤標準淨利率",
    ];
    // let str_names = vec![
    //     "code",
    //     "name",
    //     "pure_profit_rate",
    //     "income_standard",
    //     "profit_rate",
    //     "cost_rate",
    //     "net_profit_rate",
    // ];
    let mut series_cols = Vec::<Series>::new();
    for col_idx in 0..str_cols.len() {
        let ser: Series = Series::new(
            str_cols[col_idx],
            rows.iter().map(|x| x[col_idx]).collect::<Vec<_>>(),
        );
        series_cols.push(ser);
    }

    let json_string = serde_json::to_string(&code_rate_datas).unwrap();
    // println!("{:?}", json_string);

    // let title = "標準代號|小業別|擴大書審 純益率|所得額 標準|同業利潤標準毛利率|同業利潤標準費用率|同業利潤標準淨利率";
    // str_rows.insert(0, title.to_string());
    // println!("{}", str_rows.join("\n"));
    let p = Path::new(file_path);
    let mut output_file =
        File::create(format!("{}.json", p.file_stem().unwrap().to_str().unwrap())).unwrap();
    output_file.write_all(json_string.as_bytes()).unwrap();
}
