import time
import requests
import mysql.connector
from bs4 import BeautifulSoup
from colorama import init, Fore, Back, Style

db = mysql.connector.connect (
    host='localHost',
    user='aeon',
    passwd='aeon',
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
    try:
        chapter_name = []
        link_list = [] # to reset list
        hrefs = []
        old_links = []
        links_to_get = []
        old_chapter_name = []
        new_chapter_name = []


        cursor.execute('SELECT COUNT(chapter_name) FROM {}'.format(names[i])) # checking if new table
        for x in cursor:
            count = x[0]

        main_book_page = requests.get(links[i]) # getting the main page where links are
        page = main_book_page.text
        soup = BeautifulSoup(page, 'lxml')

        for tbody in soup.find_all('tbody'): # finding all the links
            for link in tbody.find_all('a'):
                link = link.get('href')
                link_list.append('https://www.royalroad.com' + link) # putting links in list for later use
                link_list = list(dict.fromkeys(link_list)) # removes duplicates

        for z in range(len(link_list)): # getting the chapter names from list
            chapter = link_list[z]
            chapter = chapter.split('/')
            chapter_name_str = chapter[-1].replace('-', '_')
            chapter_name.append(chapter_name_str)

        # print('count: {}'.format(count))
        # print('link list: {}'.format(len(link_list)))

        if count > len(link_list): # for books that have been corrupted (uploaded the chapters to amazon or something)

            cursor.execute('SELECT href FROM {}'.format(names[i])) # getting old links for compare
            for x in cursor:
                old_links.append(x[0])


            last_old_link = old_links[-1]

            how_many = len(link_list) - (link_list.index(last_old_link)) - 1 # finds the latest link that needs scraping
            if how_many != 0:

                for a in range(how_many):
                    last_ones = how_many - a

                    getting_link = link_list[-last_ones]
                    links_to_get.append(getting_link)

                    chapter = getting_link
                    chapter = chapter.split('/')
                    chapter_name_not_list = chapter[-1]
                    chapter_name_not_list = chapter_name_not_list.replace('-', '_')
                    new_chapter_name.append(chapter_name_not_list)

                    cursor.execute('INSERT INTO {} (chapter_name, href) VALUES (\'{}\', \'{}\')'.format(names[i], chapter_name_not_list, getting_link))
                    db.commit()

                for z in range(len(links_to_get)): # scraping the links
                    new_book_chapter = requests.get(links_to_get[z])
                    new_book_chapter_page = new_book_chapter.text
                    soup = BeautifulSoup(new_book_chapter_page, 'lxml')

                    content = str(soup.find_all('div', class_='chapter-inner chapter-content'))
                    soup = BeautifulSoup(content, 'lxml')
                    readable = soup.get_text()
                    readable = readable.replace('\'', '\'\'')

                    
                    print('getting chapter {}... |{}|'.format(new_chapter_name[z], names[i]))
                    cursor.execute('UPDATE {} SET content = \'{}\' WHERE chapter_name = \'{}\''.format(names[i], readable, new_chapter_name[z]))
                    db.commit()
                    print('{}chapter {} succesfully copied {}'.format(Fore.GREEN, new_chapter_name[z], Style.RESET_ALL))
                    print('{}/{}'.format(z + 1, len(links_to_get)))
                    print('\n')

                print('finished {}'.format(names[i]))
                print('\n')
                finished_chapters = names[i] + ' ' + str(len(links_to_get))


        elif count == 0: # if new table just dumps all links in there for scraping later
            for z in range(len(chapter_name)):
                cursor.execute('INSERT INTO {} (chapter_name, href) VALUES (\'{}\', \'{}\')'.format(names[i], chapter_name[z], link_list[z]))
                db.commit()

            cursor.execute('SELECT href FROM {}'.format(names[i]))
            for x in cursor:
                hrefs.append(x[0])

            for z in range(len(hrefs)): # scraping the links
                book_chapter = requests.get(hrefs[z])
                book_chapter_page = book_chapter.text
                soup = BeautifulSoup(book_chapter_page, 'lxml')

                content = str(soup.find_all('div', class_='chapter-inner chapter-content'))
                soup = BeautifulSoup(content, 'lxml')
                readable = soup.get_text()
                readable = readable.replace('\'', '\'\'')

                chapter = hrefs[z]
                chapter = chapter.split('/')
                chapter_name_not_list = chapter[-1]
                chapter_name_not_list = chapter_name_not_list.replace('-', '_')


                print('getting chapter {}... |{}|'.format(chapter_name_not_list, names[i]))
                cursor.execute('UPDATE {} SET content = \'{}\' WHERE chapter_name = \'{}\''.format(names[i], readable, chapter_name_not_list))
                db.commit()
                print('{} chpater {} succesfully copied{}'.format(Fore.GREEN, chapter_name_not_list, Style.RESET_ALL))
                print('{}/{}'.format(z + 1, len(hrefs)))
                print('\n')


            print('finished {}'.format(names[i]))
            print('\n')
            finished_chapters = names[i] + ' ' + str(len(hrefs))



        elif count < len(link_list): # for new chapters
            cursor.execute('SELECT href FROM {}'.format(names[i])) # getting old links for compare
            for x in cursor:
                old_links.append(x[0])

            for v in range(len(old_links)): # getting rid of old links
                link_list.pop(0)
            
            links_to_get = link_list

            for z in range(len(links_to_get)): # getting chapter name from links
                chapter = links_to_get[z]
                chapter = chapter.split('/')
                chapter_name_not_list = chapter[-1]
                chapter_name_not_list = chapter_name_not_list.replace('-', '_')
                chapter_name.append(chapter_name_not_list)

            cursor.execute('SELECT chapter_name FROM {}'.format(names[i])) # getting old chapter names
            for x in cursor:
                old_chapter_name.append(x[0])

            for v in range(len(old_chapter_name)): # getting rid of old chapter names in scraped links
                chapter_name.pop(0)


            for z in range(len(links_to_get)): # dumping chapter name and link for later use
                cursor.execute('INSERT INTO {} (chapter_name, href) VALUES (\'{}\', \'{}\')'.format(names[i], chapter_name[z], link_list[z]))
                db.commit()
            

            for z in range(len(links_to_get)): # scraping the new links
                new_chapter = requests.get(links_to_get[z])
                new_chapter_page = new_chapter.text
                soup = BeautifulSoup(new_chapter_page, 'lxml')

                content = str(soup.find_all('div', class_='chapter-inner chapter-content'))
                soup = BeautifulSoup(content, 'lxml')
                readable = soup.get_text()
                readable = readable.replace('\'', '\'\'')

                print('getting chapter {}... |{}|'.format(chapter_name[z], names[i]))
                cursor.execute('UPDATE {} SET content = \'{}\' WHERE chapter_name = \'{}\''.format(names[i], readable, chapter_name[z]))
                db.commit()
                print('{}chapter {} succesfully copied {}'.format(Fore.GREEN, chapter_name[z], Style.RESET_ALL))
                print('{}/{}'.format(z + 1, len(links_to_get)))
                print('\n')


            print('finished {}'.format(names[i]))
            print('\n')
            finished_chapters = names[i] + ' ' + str(len(links_to_get))

    except:
        print('fucked')