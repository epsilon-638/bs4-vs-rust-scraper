import scraper
import time

URL = 'https://en.wikipedia.org/wiki/Kurt_G%C3%B6del'

def main():
    start_time = time.time()
    print(scraper.get_all_links(URL))
    print((time.time() - start_time) * 1000)

if __name__ == "__main__":
    main()
