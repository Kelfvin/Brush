import os
import json
import lib.M_frac as M_frac
import lib.M_subject as M_subject


def show_exit_return_menu(flag = 1):
    if flag:
        print('[u] 返回上一级菜单')

    print('[q] 退出程序')
    print('*'*30)


def show_subjec_menu():
    M_frac.clear()

    print('Brush by kelf')
    print('*' * 30)
    print('题库科目：')
    base_path = './data/trace/'
    file_list = [
        f for f in os.listdir(f'{base_path}')
        if os.path.isfile(f'{base_path}{f}')
    ]           
    subject_list = []
    for f in file_list:
        m_subject = M_subject.from_trace_to_subject(f)
        subject_list.append(m_subject.get_name())

    length = len(subject_list)

    for index in range(0, length):
        print(f'[{index}] {subject_list[index]}')

    print('功能：')
    print('[i] 导入题库数据')
    print('[d] 删除题库')
    show_exit_return_menu(0)

    return subject_list


def show_section_menu(dic,fault_num,flag = 1,cc=1,r=1):
    # 传入的是字典，章节名称，对应进度
    if cc:
        M_frac.clear()  

    print('章节列表：\n')
    index = 0
    for key, value in dic.items():
        print(f'[{index}] {key} ({value})')
        index += 1

    if flag:
        print(f'[{index}] 错题 {fault_num}道')

    print('\n')
    print('功能：')
    if r:
        print('[r] 清除章节做题记录')
    print('[d] 清空错题记录')
    show_exit_return_menu()

def show_confirm_menu():
    print('[Y] 是')
    print('[N] 否')
