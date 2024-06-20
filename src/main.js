const { invoke } = window.__TAURI__.tauri;

let urlInputEl;
let urlMsgEl;
let downloadPdfs;
let downloadImages;
let scanSubfolders;
async function url_submitted() {
  urlMsgEl.textContent = "Sending request to the url: " + urlInputEl.value;

  if (urlInputEl.value == "") {
    urlMsgEl.textContent = "No url provided";
    return;
  }


  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const { url, img_box_checked, pdf_box_checked, subfolder_box_checked } = {
    url: urlInputEl.value,
    img_box_checked: downloadImages.checked,
    pdf_box_checked: downloadPdfs.checked,
    subfolder_box_checked: scanSubfolders.checked
  };

  await invoke("url_entered", { url, img_box_checked, pdf_box_checked, subfolder_box_checked });
  urlMsgEl.textContent = "Completed!";
}

window.addEventListener("DOMContentLoaded", () => {

  urlInputEl = document.querySelector("#url-input");
  urlMsgEl = document.querySelector("#url-msg");
  downloadPdfs = document.querySelector("#DownloadPdfs");
  downloadImages = document.querySelector("#DownloadImages");
  scanSubfolders = document.querySelector("#ScanSubfolders");

  document.querySelector("#url-form").addEventListener("submit", (e) => {
    e.preventDefault();
    url_submitted();
  });
});