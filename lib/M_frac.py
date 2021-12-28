import os
import lib.M_menu as M_menu
import lib.M_subject as M_subject
import pickle
import json
import sys
        
        
def clear():
    os.system('cls' if os.name == 'nt' else 'clear')

def get_message():
    return input('请输入你的选择:').strip().upper()


def erro_in_notion():
    print('错误！')
    input('输入任意键继续！')

def react_import():
    clear()
    base_path = './data/bank/'
    file_list = [f for f in os.listdir(f'{base_path}') if os.path.isfile(f'{base_path}{f}')]
    for i in file_list:
        with open(f'{base_path}{i}',mode='rt',encoding='utf-8') as file_in:
            content = file_in.read()
            m_subject = M_subject.Subject(json.loads(content))

            m_subject.to_trace()
    input('导入成功！任意键继续...')
    return 1
    


def react_num(message):
    base_path = './data/trace/'
    file_list = [f for f in os.listdir(f'{base_path}') if os.path.isfile(f'{base_path}{f}')]
    if message >= 0 and message <len(file_list): 
        with open(f'{base_path}{file_list[message]}','rb') as file_in:
            m_subject = pickle.loads(file_in.read())

            feed_back = m_subject.study()
            m_subject.to_trace()
            if(feed_back == 0):
                sys.exit(0)

    else:
        erro_in_notion()

def delete_subject():
    clear()
    base_path = './data/trace/'
    sub_list =  os.listdir(base_path)
    print('科目列表：')
    index = 0;
    for sub in sub_list:
        print(f'[{index}] {sub}')
        index+=1

    print('功能：')
    M_menu.show_exit_return_menu()
    
    choice = input('输入你要删除的科目序号：')
    if choice.upper() == 'U':
        return 1

    elif choice.upper() =='Q':
        return 0

    else:
        try:
            clear()
            print('你确定要删除吗？')
            M_menu.show_confirm_menu()
            confirm = get_message()
            if confirm == 'Y':
                os.remove(f'{base_path}{sub_list[int(choice)]}')
                return 1
        except:
            erro_in_notion()
            return 1



def react(message):
    if message == 'I':
        if not react_import():
            sys.exit(0)

    elif message == 'Q':
        sys.exit(0)
    
    elif message == 'D':

        if not delete_subject():
            sys.exit(0)

    else:
        try:
            react_num(int(message))

        except Exception:
            erro_in_notion()


    



        


            
        
        
            

