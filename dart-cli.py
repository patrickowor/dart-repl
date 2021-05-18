#! usr/bin/python3
import os
url = 'https://api.dart.dev'
def help(search, site):
    s_dic = {
        'save':'the .save key is used to save already written down code in the directory path ',
        'help':'the .help is used to search through this simple help for results ',
        'dart2js':'this is a simple tool to convert your dart script into reasonable javascript code for old browsers ',
        'clear':'type in .clear after an error occurs to clear it off the memory',
        'about':"this simple reply is created by owor Patrick and I am in no way the owner of the dart programming language or any of it tools",
        'dart':"dart is a new age object oriented programming language for building multi platform front end pages please refer to dart official website at \n" + site
    }
    result = 'sorry wrong search keyword \nhelp ==> ' + str(s_dic['help']) if s_dic.get(search, None) == None else search + ' ==> ' + s_dic[search]
    return result 

def check_bracket(code):
    curly = 0
    circly = 0
    square = 0
    back_curly = 0
    back_square = 0
    back_circly = 0
    for i in code:
        if i == "{":
            curly += 1
        elif i == "[":
            square += 1
        elif i == "(":
            circly += 1
        elif i == "}":
            back_curly += 1
        elif i == "]":
            back_square += 1
        elif i == ")":
            back_circly += 1
    if circly == back_circly and curly ==  back_curly and square == back_square:
        return False
    else:
        return True

os.system('dart --version')
print('DART REPL 0.0.2 built on \npython 3 frame and dart by owor Patrick \n .exit to cancel \n .clear to clear errors in memory and \n .help for help')
old = ''
imp = ''
while True:
    code = input('>>>')
    if code[0] == '.':
        if code == '.help':
            while True:
                search = input('--> enter keyword: ')
                if search == '.exit':
                    code = ' '
                    break
                else:
                    code = ' '
                    print(help(search,url))
        elif code == '.exit':
            if os.path.isfile('repl.dart'):
                os.remove('repl.dart')
            break
        elif code == '.dart2js':
            print('NOTE: this doesnt create a .js file only a minified all in one js file')
            name = input('filename: ')
            if os.path.isfile('repl.dart'):
                os.rename('repl.dart',new)
                code = 'dart2js --output-type=dart --minify -oapp.complete.dart' + new
                os.system("dart" + code)
            code = ''

        elif code == '.clear':
            code = '' 
            old = '' 
            print("cleared")
            continue
        if code == '.save':
            import shutil
            new = input('=> enter file name: ')
            location = input('=> enter location or directory: ')
            if location != '' and location[-1] != '/':
                location += location + '/'
            base_dir = os.getcwd()
            try:
                dest = location+'repl.dart'
                shutil.copy('repl.dart',location)
                os.rename(dest,new)
                print('=> success ' + dest)
            except:
                print("=> couldn't find directory")
                os.rename('repl.dart',new)
                print('=> file created in current directory instead')
            code = ''
    while check_bracket(code):
        new = input('...') 
        if new == '.exit':
            break
        else:
            code += new
                 
    with open('repl.dart', 'w') as file:
        if code != '' and code != ' ' and code[0] != '.' and ';' not in code[-1]:
            code += ';'
        if code.split()[0] == 'import':
            imp += code
            code = ''
        old += code
        file.write(imp.replace(';','; \n'))
        file.write("void main() {" + '\n')
        file.write(old.replace(';','; \n'))
        file.write('}')
    os.system('dart --enable-asserts repl.dart')
    if 'assert(' in code:
        result = code.split('(')
        if result[0] == 'assert' or result[0] == ' assert':
            old = old.replace(code, '')
    if 'print(' in code:
        result = code.split('(')
        if result[0] == 'print' or result[0] == ' print':
            old = old.replace(code,'')

    