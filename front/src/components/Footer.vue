<template>
    <footer>
        <p>© {{ ano }} - Todos os direitos reservados</p>
    </footer>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useRoute } from 'vue-router';

export default defineComponent({
    name: 'Footer',
    setup() {
        const route = useRoute();
        return {
            route
        }
    },
    data() {
        return {
            ano: new Date().getFullYear()
        }
    }
});

</script>

<style scoped>

footer {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 1rem;
    background-color: var(--secundary-color);
    color: rgb(197, 197, 197);
    bottom: 0;
    height: 100px;
    box-shadow: 0px -2px 5px 0px rgba(0,0,0,0.75);
}

</style>