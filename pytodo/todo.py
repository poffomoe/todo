#aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
todo_list = open("todo.py", "r"{})

while True:
    print("\nTo-Do List Menu:")
    print("1. Add task")
    print("2. View tasks")
    print("3. Mark task as done")
    print("4. Exit")
    
    choice = input("Enter your choice: ")
    
    if choice == "1":
        task = input("Enter a new task: ")
        priority = input("Enter task priority (high, medium, low): ")
        todo_list[task] = priority
        print("Task added to the list.")
    elif choice == "2":
        print("To-Do List:")
        for task, priority in todo_list.items():
            print(f"Task: {task} (Priority: {priority})")
    elif choice == "3":
        if todo_list:
            task = input("Enter the task to mark as done: ")
            if task in todo_list:
                del todo_list[task]
                print(f"Task '{task}' marked as done.")
            else:
                print("Task not found in the list.")
        else:
            print("To-Do List is empty.")
    elif choice == "4":
        print("Exiting the to-do list application.")
        break
    else:
        print("Invalid choice. Please try again.")