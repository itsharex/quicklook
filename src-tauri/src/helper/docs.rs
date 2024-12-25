use std::{fs::File, io::Read};
use calamine::Reader;
use docx_rs::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Docs {
    Excel(Vec<DSheet>),
    Docx(String),
}

impl Docs {
    pub fn excel(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let target = excel(file_path)?;
        Ok(Docs::Excel(target))
    }
    pub fn csv(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let target = csv(file_path)?;
        Ok(Docs::Excel(target))
    }
    pub fn docx(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // let target = docx(file_path)?;
        // todo 解析工作目前由 web 端解析
        Ok(Docs::Docx(file_path.to_string()))
    }
    
}

#[derive(Debug, Clone, Serialize)]
pub struct DSheet {
    name: String,
    rows: Vec<Vec<String>>,
}

fn excel(file_path: &str) -> Result<Vec<DSheet>, Box<dyn std::error::Error>> {
    let mut workbook = calamine::open_workbook_auto(file_path)?;
    let sheets = workbook.sheet_names().to_owned();
    let mut target: Vec<DSheet> = Vec::new();

    for sheet in sheets {
        let range = workbook.worksheet_range(&sheet)?;
        let mut rows = Vec::new();
        for row in range.rows() {
            let mut cell_list: Vec<String> = Vec::new();
            for cell in row.iter() {
                cell_list.push(cell.to_string());
            }
            rows.push(cell_list);
        }
        let map = DSheet { name: sheet, rows };

        target.push(map);
    }
    Ok(target)
}
fn csv(file_path: &str) -> Result<Vec<DSheet>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut rows: Vec<Vec<String>> = vec![];
    for result in rdr.records() {
        let record = result?;
        let mut cols: Vec<String> = Vec::new();
        for i in 0..record.len() {
            cols.push(record[i].to_string());
        }
        rows.push(cols);
    }
    let target: Vec<DSheet> = vec![DSheet {
        name: "sheet1".to_string(),
        rows,
    }];
    Ok(target)
}
fn docx(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(&file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let instance = read_docx(&buf)?;

    let body = instance.document.children;


    let content = body.iter().map(|x| {
        match x {
            DocumentChild::Paragraph(p) => {
                // 如何解析成html

                println!("{:?}", p.children);
                format!("<p>{}</p>", p.raw_text())
            },
            _ => "".to_string(),
        }
    }).collect::<Vec<String>>().join("\n");
    
    Ok(content)
}

struct Docx2Html {
    document: String,
    styles: String,
}

impl Docx2Html {
    fn new() -> Self {
        Docx2Html {
            document: "".to_string(),
            styles: "".to_string(),
        }
    }
    fn prase_paragraph(&self, p: &Box<Paragraph>) -> String {
        let content = p.raw_text();
        format!("<p>{}</p>", content)
    }

    fn parse_table(&self, t: &Box<Table>) -> String {
        let mut content = String::new();
        for row in t.rows.iter() {
            content.push_str("<tr>");
            let row = match row {
                TableChild::TableRow(r) => r,
            };
            for cell in row.cells.iter() {
                content.push_str(&format!("<td>{:?}</td>", cell));
            }
            content.push_str("</tr>");
        }
        format!("<table>{}</table>", content)
    }
}