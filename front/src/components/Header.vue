<template>
    <header>
        <router-link class="div" :to="{ name: 'Home' }">
            <img :src="logo" alt="logo">
        </router-link>
        <nav>
            <ul>
                <li>
                    <a class="a" href="https://github.com/fnxln/brafhy" target="_blank">GitHub</a>
                </li>
                <li>
                    <router-link :class="getLinkClass('Home')" :to="{ name: 'Home' }">Home</router-link>
                </li>
                <li>
                    <router-link :class="getLinkClass('Login')" :to="{ name: 'Login' }">Login</router-link>
                </li>
            </ul>
        </nav>
    </header>
</template>

<script lang="ts">
import { useRoute } from 'vue-router';
import { defineComponent } from 'vue';
import logo from '@/assets/logoS.png';

export default defineComponent({
    name: 'Header',
    setup() {
        const route = useRoute();
        return {
            route
        }
    },
    data() {
        return {
            logo: logo,
        }
    },
    methods: {
        getLinkClass(routeName: string) {
            return this.route.name === routeName ? 'btn' : 'a';
        }
    }
});

</script>

<style scoped>
header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 70px;
    padding: 0 300px;
    width: 100vw;
    background-color: var(--secundary-color);
}

header .div {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 70%;
}

header .div img {
    height: 100%;
}

header nav {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 100%;
}

header nav ul {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 100%;
}

header nav ul li {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 100%;
    margin-left: 20px;
}

header nav ul li .a {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    height: 100%;
    color: var(--secondary-color);
    text-decoration: none;
    font-size: 20px;
    font-weight: 500;
    content: " ";
}

header nav ul li .a::after {
    content: " ";
    width: 0%;
    height: 2px;
    background-color: var(--primary-color);
    transition: width 0.3s;
}

header nav ul li .a:hover {
    color: var(--primary-color);
}

header nav ul li .a:hover::after {
    width: 100%;
}

header nav ul li .btn {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    color: var(--primary-color);
    text-decoration: none;
    font-size: 20px;
    font-weight: 500;
    background-color: transparent;
    border: none;
    cursor: pointer;
    content: " ";
}

header nav ul li .btn::after {
    content: " ";
    width: 100%;
    height: 2px;
    background-color: var(--primary-color);
    transition: width 0.3s;
}


@media screen and (max-width: 1400px) {
    header {
        padding: 0 200px;
    }
}

@media screen and (max-width: 1250px) {
    header {
        padding: 0 100px;
    }
}

@media screen and (max-width: 1000px) {
    header {
        padding: 0 50px;
    }
}

@media screen and (max-width: 900px) {
    main {
        flex-direction: column-reverse;
        height: auto;
        padding: 20px 0px;
    }
}

@media screen and (max-width: 800px) {
    header {
        padding: 0 20px;
    }

    header div {
        height: 50%;
    }
}

@media screen and (max-width: 500px) {
    header div {
        height: 50%;
        width: 40%;
    }
}

@media screen and (max-width: 600px) {
    header div {
        height: 40%;
    }

    header nav {
        height: 50%;
        width: 70%;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    header nav ul {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: space-around;
    }

    header nav ul li {
        margin-left: 10px;
    }

    header nav ul li .a {
        font-size: 15px;
    }

    header nav ul li .btn {
        font-size: 15px;
        padding: 4px 8px;
    }
}

@media screen and (max-width: 450px) {
    header div {
        height: 30%;
        width: 30%;
    }

    header nav {
        height: 50%;
        width: 70%;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
}
</style>