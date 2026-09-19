from pathlib import Path
import requests
import time

API_URL = "https://commons.wikimedia.org/w/api.php"
CATEGORY = "Category:SVG_Oblique_illustrations_of_Mahjong_tiles"
OUTPUT_DIR = Path("mahjong_svgs")

OUTPUT_DIR.mkdir(exist_ok=True)

session = requests.Session()
session.headers["User-Agent"] = "MahjongSVGDownloader/1.0"


def iter_category_svgs(category):
    gcmtoken = None

    while True:
        params = {
            "action": "query",
            "format": "json",
            "generator": "categorymembers",
            "gcmtitle": category,
            "gcmtype": "file",
            "gcmlimit": "500",
            "prop": "imageinfo",
            "iiprop": "url",
        }

        if gcmtoken:
            params["gcmcontinue"] = gcmtoken

        r = session.get(API_URL, params=params)
        r.raise_for_status()

        data = r.json()

        for page in data.get("query", {}).get("pages", {}).values():
            imageinfo = page.get("imageinfo")
            if not imageinfo:
                continue

            yield page["title"], imageinfo[0]["url"]

        if "continue" not in data:
            break

        gcmtoken = data["continue"]["gcmcontinue"]


def download(title, url):
    filename = title.removeprefix("File:")
    filepath = OUTPUT_DIR / filename

    if filepath.exists():
        print(f"Skipping {filename}")
        return


    print(f"Downloading {filename}")

    with session.get(url, stream=True) as r:
        r.raise_for_status()

        with open(filepath, "wb") as f:
            for chunk in r.iter_content(8192):
                f.write(chunk)
    time.sleep(10)


for title, url in iter_category_svgs(CATEGORY):
    try:
        if url.lower().endswith(".svg"):
            download(title, url)
    except Exception as e:
        print(e)
        time.sleep(300)

print("Done")
