import { BrowserWindow, app } from "electron";
import express from "express";

const server = express();

server.get("/scrape", (req, res) => {
  console.log("Scraping...");

  const url = req.query.url;

  const win = new BrowserWindow({ show: false });

  win.webContents.session.clearStorageData();
  win.webContents.session.clearCache();

  win.loadURL(url.toString());
  win.webContents.on("did-finish-load", () => {
    win.webContents
      .executeJavaScript(
        `
        [
          document.querySelector("h3.sub-nav-cta__header").innerHTML,
          document.querySelector("div.show-more-less-html__markup").innerText,
          document.querySelector("a.sub-nav-cta__optional-url").innerHTML
        ]
      `
      )
      .then((data) => {
        res.json({
          title: data[0] || "",
          description: data[1] || "",
          poster: data[2] || "",
        });
        console.log("Scraping done");
      })
      .catch((err) => {
        console.log("Scraping error", err);
        res.sendStatus(500);
      })
      .finally(() => {
        win.close();
        console.log("Scraping window closed");
      });
  });
});

server.listen(8383, () => {
  console.log("Server listening on port 8383");
});

app.on("window-all-closed", (e: Event) => {
  e.preventDefault();
});
