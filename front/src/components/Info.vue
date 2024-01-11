<template>
    <div class="container">
        <div class="box">
            <div>
                <h1>Brafhy</h1>
                <h2>Brafhy is an open-source real-time messaging application built in Rust and Vue.js, featuring WebSocket and PostgreSQL.</h2>
            </div>
        </div>
        <div class="box">
            <img :src="img" alt="logo">
        </div>
    </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue';
import Img from '@/assets/logo.png';

export default defineComponent({
    name: 'Info',
    data() {
        return {
            img: Img,
        }
    }
});

</script>
<style scoped>
.container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 500px;
    min-width: 100vw;
    background-color: rgba(255, 255, 255, 0.027);
}

.container .box {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100%;
    width: 50%;
    padding: 0 100px;
}

.container .box div {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    height: 100%;
    width: 60%;
}

.container .box h1 {
    font-size: 50px;
    font-weight: 500;
    color: var(--secondary-color);
    margin-bottom: 20px;
}

.container .box h2 {
    font-size: 30px;
    font-weight: 500;
    color: var(--secondary-color);
    margin-bottom: 20px;
}

.container .box div a {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    padding: 10px 20px;
    border-radius: 50px;
    text-decoration: none;
    font-size: 20px;
    font-weight: 500;
    background-color: transparent;
    color: var(--primary-color);
    border: solid var(--primary-color) 1px;
    cursor: pointer;
}

.container .box div a:hover {
    color: var(--secondary-color);
    background-color: var(--primary-color);
    border: solid transparent 1px;
}

.container .box img {
    width: 60%;
}

.transition {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 120px;
    overflow: hidden;
}

.transition img {
    height: inherit;
    -o-object-fit: cover;
    object-fit: cover;
    -o-object-position: top;
    object-position: top;
    opacity: .8;
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    right: 0;
    width: 100%;
    height: 100%;
}

.transition::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: -1px;
    background: linear-gradient(0deg, #0f0f0f, transparent);
}

section{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 500px;
    background-color: rgba(255, 255, 255, 0.027);
}

@media screen and (max-width: 1400px) {
    .container .box div a{
        font-size: 18px;
    }
}

@media screen and (max-width: 1250px) {
    .container .box div a{
        font-size: 16px;
    }

    .container .box div{
        width: 80%;
    }

    .container .box h1{
        font-size: 40px;
    }

    .container .box h2{
        font-size: 25px;
    }

    .container .box img{
        width: 70%;
    }

}

@media screen and (max-width: 1080px) {
    .container .box img{
        width: 80%;
    }
}

@media screen and (max-width: 1000px) {
    .container .box div a{
        font-size: 14px;
    }

    .container .box div{
        width: 90%;
    }

    .container .box h1{
        font-size: 35px;
    }

    .container .box h2{
        font-size: 20px;
    }

    .container .box img{
        width: 90%;
    }
}

@media screen and (max-width: 900px) {
    .container{
        flex-direction: column-reverse;
        height: auto;
        padding: 20px 0px;
    }

    .container .box{
        width: 100%;
        height: auto;
        padding: 20px 20px;
    }

    .container .box div{
        width: 100%;
        height: auto;
        padding: 20px 0px;
    }

    .container .box h1{
        font-size: 30px;
    }

    .container .box h2{
        font-size: 18px;
    }

    .container .box img{
        width: 30%;
    }
}

@media screen and (max-width: 700px) {
    .container .box img{
        width: 40%;
    }
}

@media screen and (max-width: 600px) {
    .container .box img{
        width: 40%;
    }

    .container .box img{
        width: 50%;
    }
}
</style>