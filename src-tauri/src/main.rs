// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

static CURRENT_DIRECTORY : &str = "Download/";
// std(s)
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::path::Path;

// use(s)
use reqwest::Client;
use colored::Colorize;
use scraper::{Html, Selector};
use serde::Serialize; // Required for returning values from Tauri commands
use std::error::Error; // Required for error handling

async fn get_table(url: &str, file_path: &str, download_pdfs:bool, download_imgs:bool, scan_subfolders:bool) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("URL : {}", url.bold().blink()); // Print the URL

    let response = ureq::get(url).call()?; // send a request to the url
    let html = Html::parse_document(&response.into_string()?); // parse the html from the response
    let table_selector = Selector::parse("table").unwrap(); // make table selector
    
    // Get all tables from html
    for table in html.select(&table_selector){ // get all tables from html

        let row_selector = Selector::parse("tr").unwrap(); // make a row selector
        
        // Get all rows from table
        for row in table.select(&row_selector) { // get all rows from table

            let href_selector = Selector::parse("a[href]").unwrap(); // make a href selector

            for href in row.select(&href_selector) { // Get all links from row

                let href_attr = href.value().attr("href").unwrap(); // gets the href attribute
                
                let img_selector = Selector::parse("img").unwrap();

                for img in row.select(&img_selector) {// get all images from row

                    let is_directory = "[DIR]";
                    let is_image = "[IMG]";
                    let is_pdf = "[   ]";
                    let is_parent_directory = "[PARENTDIR]";
                    let is_icon = "[ICO]";

                    if let Some(alt) = img.value().attr("alt") {
                        if alt == is_parent_directory || alt == is_icon{
                        }
                        else{

                            let href_link = url.to_string() + href_attr; // Link obtained by looking inside the url
                            let file_name = href_attr.split('/').last().unwrap_or("unknown");
                            let _file_type = href_attr.split('.').last().unwrap_or("unknown");
                            let folder = file_path.to_string() +
                                (href_attr.split('/').take(2).collect::<Vec<&str>>().join("/")).as_str();
                            let folder_to_download = folder.replace(file_name, "");

                            println!("Link: {}", href_link.bright_green().bold());

                            let href_attr = href.value().attr("href").unwrap();
                            // if href_attr == "?C=N;O=D" || href_attr == "?C=M;O=A" || href_attr == "?C=S;O=A" || href_attr == "?C=D;O=A" {
                            //     println!("{}","Extra btns".bright_cyan());
                            // }
                            // else was here
                            if alt == is_directory {
                                if scan_subfolders{

                                        fs::create_dir_all(CURRENT_DIRECTORY.to_string() + href_attr).unwrap_or_else(|why| {
                                            println!("! {:?}", why);
                                        });
                                        // STILL Needs
                                        // CURRENT_DIRECTORY = (CURRENT_DIRECTORY.to_string() + href_attr).as_str();
    
                                        // Bcz it can get infinitelty long so we use box::pin
                                        Box::pin(get_table((url.to_string() + href_attr).as_str(), folder.as_str(),
                                         download_pdfs, download_imgs, scan_subfolders)).await?; // Call get_table function with new url
                                }
                               
                            }

                            else if alt == is_image{
                                if download_imgs{
                                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download).await?;
                                }
                                else{
                                    println!("Found img but, didnt download");
                                }
                        
                            }
                            else if alt == is_pdf{
                                if download_pdfs{
                                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download).await?;
                                }
                                else{
                                    println!("Found pdf but, didnt download");
                                }
                                
                            }
                            else{
                                println!("{}{}",url.bright_yellow(), href_attr.bright_yellow());
                            }
                        }
                        


                    }// if let Some(alt) = img.value().attr("alt")
                }// for img

                // <--------------> Code for http://www.tajalli.in/pdfs.asp <-------------------->
                
                // let file_name = href_attr.split('/').last().unwrap_or("unknown");
                // let file_type = href_attr.split('.').last().unwrap_or("unknown");
                // let folder = "Download/".to_string() +
                //     (href_attr.split('/').take(2).collect::<Vec<&str>>().join("/")).as_str() + "/";
                // if file_type == "asp"{

                // }
                // else {

                //     let parent_url = url.replace("pdfs.asp", "");

                //     let folder_to_create = folder.as_str();
                //     create_directory_if_it_does_not_exist(&folder_to_create);
                    
                //     let file_to_download = parent_url + href_attr;

                //     println!("URL inide href: {} | File Name: {} | File Type: {} | Folder {}, Link {}",
                //         href_attr.bright_green(),
                //         file_name.purple(),
                //         file_type.bright_yellow(),
                //         folder.red(),
                //         file_to_download.cyan()
                //     );

                //     // if we fix this function the whole code will work for tajalli only
                //     download_file_from_url_with_folder(file_to_download.as_str(), &folder).await?;
                // };
                

                // This code will download files other than dl.chughtailibrary.com
                // download_file(text, file_name, file_type, path);
                
            }// for href

            // println!("{}", row.inner_html().bright_green());

        }// for row

        // println!("{}" , table.inner_html().red());

    }// for table

    Ok(None) // return None
}

async fn download_file_from_url_with_folder(url : &str, input_path:&str) -> Result<(), Box<dyn std::error::Error>> {

    create_directory_if_it_does_not_exist(input_path);

    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    let file_name = url.split('/').last().unwrap_or("unknown");

    let file_type = file_name.split('.').last().unwrap_or("unknown");

    let path = input_path.to_string() + file_name;
    // let path_to_download = (CURRENT_DIRECTORY.to_string() + "/" +  file_name);
    let mb = bytes.len() / (1024 * 1024);

    println!("{} {} | {} {} | {} {} MB | Path {}",
    //  Headings in bold     variables with colors
        "File Type:".red().underline(), file_type.bold().bright_purple(),
        "File Name:".green().underline(), file_name.bold().bright_yellow(),
        "File Size:".blue().underline(), mb.to_string().bold().bright_cyan(),
        path.magenta()
    );

    println!("{} | {}","Downloading at".underline().bold(), path);

    let file_path = Path::new(&path); // added &
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;

    Ok(())
}

fn create_directory_if_it_does_not_exist(directory_path: &str) {
    if !fs::metadata(directory_path).is_ok() {
        fs::create_dir_all(directory_path).unwrap_or_else(|why| {
            println!("! {:?}", why);
        });
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//                                                                                                 Return        Result<String, ()>
#[tauri::command(rename_all = "snake_case")]
async fn url_entered(url: String, img_box_checked:bool, pdf_box_checked:bool, subfolder_box_checked:bool) -> Result<String, ()> {
    println!("IMAGE:{} | PDF:{} | Folder:{} | URL: {}", img_box_checked, pdf_box_checked, subfolder_box_checked, url);


    let _ = get_table(&url, CURRENT_DIRECTORY, pdf_box_checked, img_box_checked, subfolder_box_checked).await;
    

    Ok(format!("AT UI: {}", url))
}

#[tauri::command]
async fn test(test_var: &str) -> Result<String, ()>{
    println!("TEST DONE:{}", test_var);
    Ok(format!("AT UI: {}", test_var))
}

fn main() {
    // control::set_virtual_terminal(true).unwrap();
    println!("Hello, world!");
    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![
                // all commands go here
                url_entered,
                test
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

