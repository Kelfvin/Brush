import json
import argparse

def migrate_json_data(path:str,save_file:str):
    with open (path, 'r', encoding='utf-8') as file:
        data = json.load(file)

    type_map = {
        "多选题": "MC",
        "单选题": "SC",
    }

    for section in data.get("sections", []):
        for question in section.get("problems", []):
            if question.get("type") in type_map:
                question["type"] = type_map[question["type"]]

            _key = question.get("key", None)
            if _key:
                question["key"] = list(_key)

            else:
                question["key"] = []
                print(f"Warning: Question '{question.get('title', 'Unknown')}' has no key. Setting to empty list.")



    # 修改 problems 为 questions
    for section in data.get("sections", []):
        if "problems" in section:
            section["questions"] = section.pop("problems")



    with open (save_file, 'w', encoding='utf-8') as file:
        json.dump(data, file, ensure_ascii=False, indent=4)
            

def main():
    parser = argparse.ArgumentParser(description="Migrate JSON data format.")
    parser.add_argument("--input", type=str, help="Path to the input JSON file.")
    parser.add_argument("--output", type=str, help="Path to save the migrated JSON file.")
    args = parser.parse_args()

    migrate_json_data(args.input, args.output)    


if __name__ == "__main__":
    main()