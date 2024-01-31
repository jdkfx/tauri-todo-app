<template>
    <div id="todoContainer">
        <h1>TODOアプリ</h1>
        <form @submit.prevent="addTodo">
            <input v-model="newTodo" placeholder="新しいTODOを追加">
            <button>追加</button>
        </form>

        <ul>
            <li v-for="(todo, index) in todos">
                {{ todo.title }}
                <button @click="removeTodo(index)">削除</button>
            </li>
        </ul>
    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "TodoList",
    components: {},
    data() {
        return {
            newTodo: "",
            todos: [
                {
                    title: "Rustを勉強する"
                },
                {
                    title: "PHPを勉強する"
                },
            ],
        };
    },
    methods: {
        async addTodo() {
            if (this.newTodo.trim() !== "") {
                this.todos.push({
                    title: this.newTodo
                })
                console.log(this.todos);
                // invoke("add_todo", { title: this.newTodo.trim() }).then((response) => {
                //     console.log(response);
                // });
                this.newTodo = "";
            }
        },
        removeTodo(index) {
            this.todos.splice(index, 1);
            // invoke("remove_todo", index).then((response) => {
            //     console.log(response);
            // });
            console.log(this.todos);
        }
    },
}
</script>
