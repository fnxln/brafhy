<template>
    <div class="login">
        <Header />
        <main>
            <div class="box" id="1">
                <img :src="logo" alt="logo" />
            </div>
            
            <div class="box" id="2">
                <h2>Login</h2>
                <FormKit type="email" label="Your Email" help="Insira seu email para continuar" validation="required|email"
                validation-visibility="live" placeholder="example@email.com" v-model="requestModel.email" />
            <FormKit type="group">
                <h2>Create a new password</h2>
                <FormKit type="password" name="password" label="Password" help="Enter a new password" validation="required"
                    validation-visibility="live" v-model="requestModel.password" />
                <FormKit type="password" name="password_confirm" label="Confirm password" help="Confirm your new password"
                    validation="required|confirm" validation-visibility="live" validation-label="Password confirmation" />
            </FormKit>
            <FormKit type="submit" label="Login" @click="login" /></div>
            
        </main>
        <Footer />
    </div>
</template>

<script lang="ts">
import Header from '../components/Header.vue';
import Footer from '../components/Footer.vue';
import { defineComponent } from 'vue';
import api from '../services/api';
import { LoginResponse } from '../services/dto';
import logo from '@/assets/logo.png';

export default defineComponent({
    name: 'Login',
    data() {
        return {
            requestModel: {
                email: '',
                password: ''
            },
            logo: logo
        };
    },
    methods: {
        async login() {
            try {
                const response = await api.post('/user/login', this.requestModel, { withCredentials: true });
                console.log(response.data  as LoginResponse );
                // Handle the response, e.g., redirecting the user
            } catch (error) {
                console.error('Login error:', error);
                // Handle the error, e.g., show an error message to the user
            }
        }
    },
    components: {
        Header,
        Footer
    }
});
</script>

<style scoped>
main{
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
    height: 100vh;
    width: 100vw;
    background-color: var(--secundary-color);
}

.box{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 99%;
    width: 48%;
    border-radius: 10px;
    box-shadow: 0px 0px 10px 0px rgba(0,0,0,0.75);
}


</style>