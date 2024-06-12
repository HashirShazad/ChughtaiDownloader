// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
static mut CURRENT_DIRECTORY : &str = "Download/";
// std(s)
use std::io::{prelude::*, Bytes};
use std::fs::File;
use std::fs;
use std::io;
use std::io::BufReader;
use std::time::Instant;
use std::path::Path;
use select::document::Document; 

// use(s)
use futures::stream::StreamExt;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use colored::Colorize;
use tokio::sync::broadcast::error::SendError;
use scraper::{Html, Selector};
use colored::*;

async fn get_table(url: &str, file_path: &str, download_pdfs:char, download_imgs:char, scan_subfolders:char) -> Result<Option<String>, Box<dyn std::error::Error>> {
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
                            let file_type = href_attr.split('.').last().unwrap_or("unknown");
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
                                if scan_subfolders == 'y' || scan_subfolders == 'Y'{
                                    unsafe
                                    {
                                        fs::create_dir_all((CURRENT_DIRECTORY.to_string() + href_attr)).unwrap_or_else(|why| {
                                            println!("! {:?}", why);
                                        });
                                        // STILL Needs
                                        // CURRENT_DIRECTORY = (CURRENT_DIRECTORY.to_string() + href_attr).as_str();
    
                                        // Bcz it can get infinitelty long so we use box::pin
                                        Box::pin(get_table((url.to_string() + href_attr).as_str(), folder.as_str(),
                                         download_pdfs, download_imgs, scan_subfolders)).await?; // Call get_table function with new url
                                    }
                                }
                               
                            }

                            else if alt == is_image{
                                if download_imgs == 'y' || download_imgs == 'Y'{
                                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download).await?;
                                }
                                else{
                                    println!("Found img but, didnt download");
                                }
                        
                            }
                            else if alt == is_pdf{
                                if download_pdfs == 'y' || download_pdfs == 'Y' {
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
                // }
                

                // This code will download files other than dl.chughtailibrary.com
                // download_file(text, file_name, file_type, path);
                
            }// for href

            // println!("{}", row.inner_html().bright_green());

        }// for row

        // println!("{}" , table.inner_html().red());

    }// for table

    Ok(None) // return None
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
