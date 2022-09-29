import requests as req
from bs4 import BeautifulSoup


def get_href(anchor):
    return str(anchor.get('href'))


def get_all_links(url):
    response = req.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')

    anchors = soup.find_all('a')
    hrefs = filter(lambda url: url != 'None', map(get_href, anchors))

    return list(hrefs)
