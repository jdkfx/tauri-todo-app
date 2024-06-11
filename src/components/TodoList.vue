<template>
    <div id="todoContainer">
        <h1>TODOアプリ</h1>
        <form @submit.prevent="addTodo">
            <input v-model="newTodo" placeholder="新しいTODOを追加">
            <button>追加</button>
        </form>

        <p>{{ resultMsg }}</p>

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
import { ref } from "vue";

export default {
    name: "TodoList",
    components: {},
    data() {
        return {
            newTodo: ref(""),
            todos: ref([
                {
                    title: "Rustを勉強する"
                },
                {
                    title: "PHPを勉強する"
                },
            ]),
            resultMsg: ref(""),
        };
    },
    methods: {
        async addTodo() {
            if (this.newTodo.trim() !== "") {
                this.todos.push({ title: this.newTodo });
                this.resultMsg = await invoke("add_todo", { title: this.newTodo.trim() });
                this.newTodo = "";
            }
        },
        async removeTodo(index) {
            this.todos.splice(index, 1);
            this.resultMsg = await invoke("remove_todo");
        }
    },
}
</script>
