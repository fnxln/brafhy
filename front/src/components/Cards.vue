<template>
    <!-- quero cards na minha pagina um do lado do outro em forma de retangulos em pé -->
    <H1>
        Tecnologias utilizadas no projeto:
    </H1>
    <div class="cards">
        <div class="card" id="card1">
            <div class="frente">
                <h3>Vue + Vite</h3>
            </div>
            <div class="verso">
                <p>
                </p>
            </div>

        </div>
        <div class="card" id="card2">
            <div class="frente">
                <h3>API Rust</h3>
            </div>
            <div class="verso">
                <p>
                </p>
            </div>
        </div>
        <div class="card" id="card3">
            <div class="frente">
                <h3>PostgreSQL</h3>
            </div>
            <div class="verso">
                <p>
                </p>
            </div>
        </div>
        <div class="card" id="card4">
            <div class="frente">
                <h3>Docker</h3>
            </div>
            <div class="verso">
                <p>
                </p>
            </div>
        </div>
    </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'Cards'
});

</script>
<style scoped>
h1 {
    text-align: center;
    padding: 10px;
    color: white;
}

.cards {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-around;
    align-items: center;
    margin: 0 auto;
    width: 100%;
    padding: 0 20px;
}

.card {
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    width: 18%;
    max-width: 18%;
    height: 400px;
    border-radius: 5px;
    padding: 20px;
    margin: 10px;
}

.card h3 {
    margin: 0;
}

.card p {
    margin: 0;
    display: flex;
    flex-direction: column;
    text-align: left;
    min-height: 70%;
}

.card .frente {
    /* sem repetir ou obrir todo o card */
    background-size: 80%;
    background-position: center;
    background-repeat: no-repeat;
}

#card1 .frente {
    background-image: url(../assets/cards/card1.png);
}

#card2 .frente {
    background-image: url(../assets/cards/card2.png);
}

#card3 .frente {
    background-image: url(../assets/cards/card3.png);
}

#card4 .frente {
    background-image: url(../assets/cards/card4.png);
}

.frente,
.verso {
    display: flex;
    justify-content: space-around;
    flex-direction: column;
    position: absolute;
    height: 100%;
    width: 100%;
    -webkit-backface-visibility: hidden;
    backface-visibility: hidden;
    transform-style: preserve-3d;
    transition: ease-in-out 600ms;
}

.frente {
    background-size: cover;
    background-color: var(--quaternary-color);
    color: var(--secondary-color);
    overflow: hidden;
    font-family: Poppins, sans-serif;
}

.frente h3 {
    padding: 1rem;
    min-height: 20%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 1.5rem;
    color: var(--secondary-color);
    background-color: rgb(0, 0, 0, 0.281);
    border-radius: 5px;
    backdrop-filter: blur(10px);
}

.frente:before {
    position: absolute;
    display: block;
    content: "";
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-image: url();
    opacity: 0.25;
    z-index: -1;
}

/* .card:hover .frente {
    transform: rotateY(180deg);
} */

/* .card:nth-child(even):hover .frente {
    transform: rotateY(-180deg);
} */

.verso {
    background: var(--primary-color);
    transform: rotateY(-180deg);
    padding: 0 2em;
}

.verso p {
    /* width: 100%; */
    float: left;
    margin-bottom: 20px;
    color: var(--secondary-color);

}

/* .card:hover .verso {
    transform: rotateY(0deg);
} */

@media screen and (max-width: 1224px) {
    .card {
        width: 40%;
        max-width: 40%;
    }
}

@media screen and (max-width: 768px) {
    .card {
        width: 80%;
        max-width: 80%;
    }
}

@media screen and (max-width: 480px) {
    .card {
        width: 100%;
        max-width: 100%;
    }
}
</style>