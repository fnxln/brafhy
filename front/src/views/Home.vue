<template>
    <div class="home">
        <Header />
        <main>
            <Slides />
        </main>
        <Info />
        <Cards />
        <Footer />
    </div>
</template>

<script lang="ts">
import Header from '../components/Header.vue';
import Footer from '../components/Footer.vue';
import Slides from '../components/Slides.vue';
import Cards from '../components/Cards.vue';
import Info from '../components/Info.vue';

import { defineComponent } from 'vue';
export default defineComponent({
    name: 'Home',
    components: {
        Header,
        Footer,
        Slides,
        Cards,
        Info
    }
})
</script>

<style></style>