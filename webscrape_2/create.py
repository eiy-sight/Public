import mysql.connector

db = mysql.connector.connect (
    host='localHost',
    user='aeon',
    passwd='aeon',
    database='books'
)

cursor = db.cursor()
tables = []
new_tables = []

cursor.execute('SELECT name FROM links')
for x in cursor:
    new_tables.append(x[0])

cursor.execute('SHOW TABLES')
for x in cursor:
    tables.append(x[0])


for i in range(len(new_tables)):
    for y in range(len(tables)):
        if new_tables[i] != tables[y]:
            cursor.execute('CREATE TABLE IF NOT EXISTS {} (chapter_name varchar(255) NOT NULL, href TEXT NOT NULL, content MEDIUMTEXT)'.format(new_tables[i]))
            db.commit()

# new_tables = []
# cursor.execute('SHOW TABLES')
# for x in cursor:
#     new_tables.append(x[0])

# for i in range(len(new_tables)):
#     if new_tables[i] in tables:
#         new_tables.remove(new_tables[i])

# print('new tables:')
# for i in range(len(new_tables)):
#     print(new_tables[i])

"""
massively bad fucking code fix it later

you dont need the broken compare function just loop through for loop
"""