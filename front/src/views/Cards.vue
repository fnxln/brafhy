<template>
    <!-- quero cards na minha pagina um do lado do outro em forma de retangulos em pé -->
    <H1>
        Por que devo usar o KEYSTONE?
    </H1>
    <div class="cards">
        <div class="card" id="card1">
            <div class="frente">
                <h3>Sistema de skins para servidores</h3>
            </div>
            <div class="verso">
                <p>
                    KEYSTONE fornece sistema de skin livre, que envolve um funcionário para fornecer mais recursos do que o
                    sistema oficial de skin tem. Nosso objetivo é criar uma oportunidade para os jogadores de clientes
                    oficiais
                    e não oficiais a instalação de skins que eles gostam e jogar em um mundo de Minecraft.

                    Agora você não precisa criar um outro sistema de skin de seu próprio gosto, você pode simplesmente usar
                    o
                    sistema de skin KEYSTONE.by universal.

                    Este sistema é livre e nunca será pago!
                </p>
            </div>

        </div>
        <div class="card" id="card2">
            <div class="frente">
                <h3>Sistema de autorização para servidor de Minecraft</h3>
            </div>
            <div class="verso">
                <p>
                    KEYSTONE fornece um serviço de autorização isomórfica, permitindo aos jogadores acessarem servidores
                    usando
                    suas contas KEYSTONE.by, evitando registros extras. Além disso, inclui oAuth para que jogadores se
                    registrem
                    em sites utilizando as mesmas contas, de maneira semelhante ao login com redes sociais como Facebook e
                    Google.
                </p>
            </div>
        </div>
        <div class="card" id="card3">
            <div class="frente">
                <h3>API - Aberto e independente</h3>
            </div>
            <div class="verso">
                <p>
                    KEYSTONE.by não pertence a qualquer servidor ou grande empresa - é um
                    projeto, desenvolvido por uma pequena equipe de programadores, que
                    não têm qualquer conhecimento sobre os princípios de design de interface
                    do usuário, mas eles fazem o seu melhor para criar o melhor que podem.
                    Nós tentamos com muita dificuldade e esperança, que você vai gostar do
                    KEYSTONE. Além disso o nosso projeto tem a documentação completa da API,
                    que permite fácil integração do KEYSTONE em seu projeto.
                </p>
            </div>
        </div>
        <div class="card" id="card4">
            <div class="frente">
                <h3>Melhor que os serviços oficiais</h3>
            </div>
            <div class="verso">
                <p>
                    KEYSTONE visa criar uma alternativa digna para os serviços oficiais relacionados a Minecraft,
                    proporcionando
                    um design mais agradável e uma interface amigável. Provavelmente já atingimos este objetivo, mas nossa
                    mente
                    voa mais rápido do que a da Mojang. Isso porque KEYSTONE oferece recursos, que você não vai encontrar em
                    um
                    sistema oficial de skin
                </p>
            </div>
        </div>
    </div>
</template>
<script>
import { defineComponent } from 'vue';
import cardimg1 from '../../public/cardimg1.jpg';

export default defineComponent({
    name: 'Cards',
    data() {
        return {
            cardimg1: cardimg1,
        };
    },
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
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
}

#card1 .frente {
    background-image: url(../../public/cardimg1.jpg);
}

#card2 .frente {
    background-image: url(../../public/cardimg2.png);
}

#card3 .frente {
    background-image: url(../../public/cardimg3.png);
}

#card4 .frente {
    background-image: url(../../public/cardimg4.png);
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

.frente h3{
    backdrop-filter: blur(10px);
    width: 100%;
    min-height: 20%;
    display: flex;
    justify-content: center;
    align-items: center;
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

.card:hover .frente {
    transform: rotateY(180deg);
}

.card:nth-child(even):hover .frente {
    transform: rotateY(-180deg);
}

.verso {
    background: var(--primary-color);
    transform: rotateY(-180deg);
    padding: 0 2em;
}

.verso p {
    width: 100%;
    float: left;
    margin-bottom: 20px;
    color: var(--secondary-color);

}

.card:hover .verso {
    transform: rotateY(0deg);
}

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