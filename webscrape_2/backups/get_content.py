import time
import requests
import mysql.connector
from bs4 import BeautifulSoup
from colorama import init, Fore, Back, Style

db = mysql.connector.connect (
    host='localHost',
    user='eiy',
    passwd='root',
    database='books'
)

cursor = db.cursor()
init()

# for later 
links = []
names = []
finished_chapters = []

# for main page
cursor.execute('SELECT link FROM links')
for x in cursor:
    links.append(x[0])

                                            # these two link up 
# for the book 
cursor.execute('SELECT name FROM links')
for x in cursor:
    names.append(x[0])


for i in range(len(links)):
    chapter_name = []
    link_list = [] # to reset list
    hrefs = []

    cursor.execute('SELECT COUNT(*) FROM {}'.format(names[i])) # checking if new table
    for x in cursor:
        count = x[0]

    main_book_page = requests.get(links[i]) # getting the main page where links are
    page = main_book_page.text
    soup = BeautifulSoup(page, 'html.parser')

    for tbody in soup.find_all('tbody'): # finding all the links
        for link in tbody.find_all('a'):
            link = link.get('href')
            link_list.append('https://www.royalroad.com' + link) # putting links in list for later use
            link_list = list(dict.fromkeys(link_list))

    for z in range(len(link_list)): # getting the chapter names from list
        chapter = link_list[z]
        chapter = chapter.split('/')
        chapter_name_str = chapter[-1].replace('-', '_')
        chapter_name.append(chapter_name_str)



    if count == 0: # if new table just dumps all links in there for scraping later
        for z in range(len(chapter_name)):
            cursor.execute('INSERT INTO {} (chapter_name, href) VALUES (\'{}\', \'{}\')'.format(names[i], chapter_name[z], link_list[z]))
            db.commit()

        cursor.execute('SELECT href FROM {}'.format(names[i]))
        for x in cursor:
            hrefs.append(x[0])

        for z in range(len(hrefs)):
            book_chapter = requests.get(hrefs[z])
            book_chapter_page = book_chapter.text
            soup = BeautifulSoup(book_chapter_page, 'html.parser')

            content = str(soup.find_all('div', class_='chapter-inner chapter-content'))
            soup = BeautifulSoup(content, 'html.parser')
            readable = soup.get_text()
            readable = readable[:-8]
            readable = readable[2:]
            readable = readable.replace('\'', '\'\'')

            chapter = hrefs[z]
            chapter = chapter.split('/')
            chapter_name = chapter[-1]
            chapter_name = chapter_name.replace('-', '_')


            print('getting chapter {}... |{}|'.format(chapter_name, names[i]))
            cursor.execute('UPDATE {} SET content = \'{}\' WHERE chapter_name = \'{}\''.format(names[i], readable, chapter_name))
            print('{}chpater {} succesfully copied{}'.format(Fore.GREEN, chapter_name, Style.RESET_ALL))
            print('{}/{}'.format(z + 1, len(hrefs)))
            print('\n')


        print('finished {}'.format(names[i]))
        finished_chapters = names[i] + ' ' + str(len(hrefs))
