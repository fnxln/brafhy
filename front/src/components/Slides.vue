<template>
    <!-- main com carrossel -->
    <div class="carrossel">
        <div class="slides">
            <div class="slide" v-for="(slide, index) in slides" :key="slide.id" :class="{ active: index === activeSlide }">
                <img :src="slide.image" :alt="slide.title">
                <div class="slide-content">
                    <h2>{{ slide.title }}</h2>
                    <p>{{ slide.description }}</p>
                </div>
            </div>
        </div>
        <div class="buttons">
            <button @click="previousSlide"></button>
            <button @click="nextSlide"></button>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Slide1Image from '@/assets/imgs/img1.jpg';
import Slide2Image from '@/assets/imgs/img2.jpg';
import Slide3Image from '@/assets/imgs/img3.jpg';
import Slide4Image from '@/assets/imgs/img4.jpg';

export default defineComponent({
    name: 'Slides',
    data() {
        return {
            slides: [
                {
                    id: 1,
                    title: 'Segurança em primeiro lugar',
                    description: 'O Brafhy tem um sistema de gerenciamento de senhas que visa a segurança de seus usuários.',
                    image: Slide1Image
                },
                {
                    id: 2,
                    title: 'Design moderno',
                    description: 'temos um design moderno e intuitivo, para que você possa usar sem dificuldades.',
                    image: Slide2Image
                },
                {
                    id: 3,
                    title: 'Multiplataforma',
                    description: 'O Brafhy é multiplataforma, ou seja, você pode usar em qualquer dispositivo.',
                    image: Slide3Image
                },
                {
                    id: 4,
                    title: 'Open Source',
                    description: 'Esse projeto é open source, ou seja, você pode contribuir com o projeto.',
                    image: Slide4Image
                }
            ],
            activeSlide: 0
        }
    },
    methods: {
        nextSlide() {
            this.activeSlide++;
            if (this.activeSlide > this.slides.length - 1) {
                this.activeSlide = 0;
            }
        },
        previousSlide() {
            this.activeSlide--;
            if (this.activeSlide < 0) {
                this.activeSlide = this.slides.length - 1;
            }
        }
    },
    mounted() {
        setInterval(() => {
            this.nextSlide();
        }, 5000);  // mudando o tempo para 3 segundos
    }
});
</script>

<!-- Aqui o resto do seu código CSS -->


<style scoped>
.carrossel {
    position: relative;
    width: 100%;
    height: 80vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.slides {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}

.slide {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    position: absolute;
    opacity: 0;
    transition: opacity 0.5s ease;
}

.slide.active {
    opacity: 1;
}

.slide img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.slide-content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    position: absolute;
    color: white;
    text-align: center;
}

.slide-content h2 {
    font-size: 3rem;
    margin-bottom: 1rem;
    backdrop-filter: blur(10px);
    padding: 1rem;
    border-radius: 5px;
    background-color: rgba(0, 0, 0, 0.281);
}

.slide-content p {
    font-size: 1.5rem;
    backdrop-filter: blur(10px);
    padding: 1rem;
    border-radius: 5px;
    background-color: rgba(0, 0, 0, 0.281);
}

.buttons {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: absolute;
    bottom: 0;
    padding: 1rem;
}

.buttons button {
    padding: 0.5rem;
    border-radius: 5px;
    border: 1px solid rgb(68, 75, 77);
    background-color: transparent;
    color: #fff;
    cursor: pointer;
    width: 50px;
    height: 100%;
    opacity: 0.2;
}

.buttons button:hover {
    background-color: rgb(53, 57, 59);
}

@media screen and (max-width: 768px) {
    .carrossel {
        height: 60vh;
    }
    .buttons {
        display: none;
    }

    .slide-content h2 {
        font-size: 2rem;
    }

    .slide-content p {
        font-size: 1rem;
    }
}
</style>