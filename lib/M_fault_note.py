# coding=utf-8
import os
import re
import pickle
import random
from lib.rich_console import console

from lib.config import problem_type_color


base_path = "./data/fault_note/"


def solve_no_note(file_name):
    if not os.path.isfile(f"{base_path}{file_name}"):
        file = open(f"{base_path}{file_name}", "wb")
        file.close()


def add_fault_note(file_name, title, type, options, key):
    solve_no_note(file_name)
    file_in = open(f"{base_path}{file_name}", mode="rb")
    content = file_in.read()
    fault_dic = {}
    if len(content):
        fault_dic = pickle.loads(content)

    if title not in fault_dic:
        temp = {"type": type, "options": options, "key": key, "review_times": 1}
        # review_times 该错题复习的次数
        fault_dic[title] = temp

        file_in.close()

        file_out = open(f"{base_path}{file_name}", mode="wb")
        pickle.dump(fault_dic, file_out)
        file_out.close()


def rmove_fault_note(file_name, title):
    solve_no_note(file_name)
    file_in = open(f"{base_path}{file_name}", mode="rb")
    content = file_in.read()
    fault_dic = {}
    if len(content):
        fault_dic = pickle.loads(content)
        review_times = fault_dic[title]["review_times"]

    if title in fault_dic:
        if fault_dic[title]["review_times"] == 3:
            # 错题复习了4次就删除
            del fault_dic[title]

        else:
            fault_dic[title]["review_times"] = review_times + 1

        file_in.close()

        file_out = open(f"{base_path}{file_name}", mode="wb")
        pickle.dump(fault_dic, file_out)
        file_out.close()
    return 1


def review_fault(file_name):
    os.system("cls" if os.name == "nt" else "clear")
    solve_no_note(file_name)
    file_in = open(f"{base_path}{file_name}", mode="rb")
    content = file_in.read()
    fault_dic = {}
    if len(content):
        fault_dic = pickle.loads(content)
        fault_dic = random_dic(fault_dic)
        # 将错题字典顺序打乱，做错题时不按照顺序

    times = 0
    for title, opt_key in fault_dic.items():
        times += 1
        type = opt_key["type"]
        options = opt_key["options"]
        key = opt_key["key"]
        review_times = opt_key["review_times"]
        # review_times 还需要复习的次数

        os.system("cls" if os.name == "nt" else "clear")
        print("错题回顾")
        print("错题复习进度({}/{})".format(times, len(fault_dic)))

        print("*" * 30)
        print(f"此题记忆进度 ({review_times}/{3})")

        console.print(f"({type})", style=problem_type_color[type], end=" ")
        print(title)
        print()

        for index, option in enumerate(options):
            letter = chr(ord("A") + index)
            print(f"{letter}: ", end="")
            print(option)

        print("[U] 返回上一级")
        print("[Q] 退出")

        print("*" * 30)

        answer = input("请输入你的选择：")
        answer = answer.replace(" ", "")
        answer = answer.replace("1", "A")
        answer = answer.replace("2", "B")
        answer = answer.replace("3", "C")
        answer = answer.replace("4", "D")
        answer = answer.replace("5", "E")
        answer = answer.replace("6", "F")
        answer = answer.upper()

        # if type == '判断题':
        #     if answer == 'A':
        #         answer = '对'

        #     elif answer == 'B':
        #         answer = '错'

        if answer == "q" or answer == "Q":
            return 0

        elif answer == "u" or answer == "U":
            return 1

        elif answer == key:
            print("正确！")
            rmove_fault_note(file_name, title)

        else:
            print("错误！  正确答案：", key)
            add_fault_note(file_name, title, type, options, key)

        input("输入任意键继续：")
        os.system("cls" if os.name == "nt" else "clear")

    choice = input("错题做完了！\n[q]退出\n[u]返回主菜单\n请输入你的选择[u|q]:")

    if choice == "q" or choice == "Q":
        return 0

    else:
        return 1


def get_fault_note_numbers(file_name):
    solve_no_note(file_name)
    file_in = open(f"{base_path}{file_name}", mode="rb")
    content = file_in.read()
    fault_dic = {}
    if len(content):
        fault_dic = pickle.loads(content)
        num = len(fault_dic)
        return num

    else:
        return 0


def reset_fault_note(file_name):
    os.remove(f"{base_path}{file_name}")


def random_dic(dict):
    dict_key_ls = list(dict.keys())
    random.shuffle(dict_key_ls)
    new_dic = {}
    for key in dict_key_ls:
        new_dic[key] = dict[key]
    return new_dic
