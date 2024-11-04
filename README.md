# PREVIEW VERSION

# ua_contact_book

Parser written with Rust and pest create to convert contact data to json format

## Parsing data
### File for parsing
Currently, the program parses data from the file used in generate_contact(). That is, test.txt

The file for parsing should have the following structure:
```
full_name; city; birth_date; phone_numbers; email
...
full_name; city; birth_date; phone_numbers; email
```
## Grammar rules
### name

This rule used to parse full_name properly: as soon, as this project oriented for ukrainian audience full_name contains Cyrillic letters, gaps and dot("Роберт Дауні мол.")

### city

This rule designed to parse contact's city: in this program city have name part(Cyrillic letters and gaps) and zip-code part(5-digit number) divided with coma and gaps
```
Київ, 00001
Бровари, 07400
```
### birth_date
This rule was writen to parse birthdate with next structure:
```
dd.mm.yyyy
dd/mm/yyyy
```
### number
Rule for parsing single ukrainian phone number. Each ukrainian number starts with 0 or +380, then non-zero digit and 8 digits after. 
```
0xxxxxxxxx
+380xxxxxxxxx
```
### numbers
As long as person can own more than 1 phone number, phone numbers can be written with coma separation and gaps
```
0xxxxxxxxx, +380xxxxxxxxx,+380xxxxxxxxx ,  +380xxxxxxxxx
```
### email
Rule for parsing email. According to standard, max email length - 254 symbols, where part before "@" up to 64 symbols, that can be ASCII_DIGIT, ASCII_ALPHA or dot.
After "@" length can be from 5 symbols up to 189, that can be ASCII_DIGIT, ASCII_ALPHA or dot.
```
user.123@examp.le
```
### contact
Rule that contain previous rules and combines it into one.
```
name; city; date; numbers; email
Олексій Ігорович; Львів, 79000; 12/04/1985; +380961234567, 0951234567; oleksii@example.com
```
### file
Rule that parse multiple contacts in file separated with NEWLINE
```
contact
contact
...
contact
```
# Processing parsed data
After parsing data converts into JSON format. Here is part of current test result
``` json
[
        {
                "full_name": Мар'яна ЄҐЇ,
                "city": Бровари, 07400,
                "birth_date": 06/06/2006,
                "contacts": {
                        "phone_numbers": ["+380904652439", "0904652439"],
                        "email": user@exam.pl
                }
        },
        {
                "full_name": Олексій Ігорович,
                "city": Львів, 79000,
                "birth_date": 12/04/1985,
                "contacts": {
                        "phone_numbers": ["+380961234567", "0951234567"],
                        "email": oleksii@example.com
                }
        },
        {
                "full_name": Катерина Михайлівна,
                "city": Ужгород, 88000,
                "birth_date": 05/05/1991,
                "contacts": {
                        "phone_numbers": ["+380939876543"],
                        "email": kateryna@example.com
                }
        }
]
```
