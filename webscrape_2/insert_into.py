import mysql.connector

db = mysql.connector.connect (
    host='localHost',
    user='aeon',
    passwd='aeon',
    database='books'
)

cursor = db.cursor()

while True:
    link = input('enter link % ')
    website = input('enter website (abbreviated) % ')
    check = input('is this the link \"{}\" % '.format(link))
    # print(link)

    if check.lower() == 'y':

        if website == 'rr':
            name = link.split('/')
            name = name[-1]
            name = name.replace('-', '_')
            print('adding {}...'.format(name))


        cursor.execute('INSERT INTO links (name, link, website) VALUES (\'{}\', \'{}\', \'{}\')'.format(name, link, website))
        db.commit()
