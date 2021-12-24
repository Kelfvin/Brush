import re
import os
import src.ans as ans
import src.proc as proc
import src.fault_note as fault_note

while(True):
    os.system('cls')

    print('菜单:','\n','*'*30)
    print('[1] 开始答题')
    print('[2] 导入题库')
    print('[3] 复习错题')
    print('[4] 清空错题记录')
    print('[q] 退出')
    print('\n','*'*30,'\n')

    model = input('请输入你的选择[1|2|3|q]:')


    if(model=='1'):
        ans.ans()

    elif(model=='2'):
        proc.proc()

    elif(model=='3'):
        fault_note.review_fault()

    elif(model=='4'):
        os.system('cls')
        file = open('./fault_note/fault_note','wb')
        file.close()
        print('已清空错题记录！')
        input()

    elif(model=='q' or model=='Q'):
        exit()




    
