<template>
    <div class="login">
        <Header />
        <main>
            <div class="box" id="box1">
                <img :src="img" alt="logo" />
            </div>
            <div class="box" id="box2">
                <h2>Login</h2>
                <FormKit 
                    type="email"
                    label="Your Email"
                    help="Insira seu email para continuar"
                    validation="required|email"
                    validation-visibility="live"
                    placeholder="example@email.com"
                    v-model="requestModel.email"
                    outer-class="w-[60%] flex flex-col justify-center items-center"
                    wrapper-class="w-full flex flex-col justify-center"
                    label-class="block mb-1 font-bold text-sm text-gray-700"
                    inner-class="border-gray-400 rounded-lg mb-2 overflow-hidden focus:outline-none m-1"
                    input-class="w-full h-10 px-3 text-base text-gray-700 placeholder-gray-400 focus:outline-none"
                    help-class="w-[70%] text-xs text-gray-500"
                    messages-class="w-[70%] text-xs text-gray-500"
                />
                <FormKit type="group">
                    <FormKit 
                        type="password"
                        name="password"
                        label="Password"
                        help="Enter a new password"
                        validation="required"
                        validation-visibility="live"
                        v-model="requestModel.password"
                        outer-class="w-[60%] flex flex-col justify-center items-center"
                        wrapper-class="w-full flex flex-col justify-center"
                        label-class="block mb-1 font-bold text-sm text-gray-700"
                        inner-class="border-gray-400 rounded-lg mb-2 overflow-hidden focus:outline-none m-1"
                        input-class="w-full h-10 px-3 text-base text-gray-700 placeholder-gray-400 focus:outline-none"
                        help-class="w-[70%] text-xs text-gray-500"
                        messages-class="w-[70%] text-xs text-gray-500"
                    />
                    <FormKit
                        type="password"
                        name="passwordConfirm"
                        label="Confirm Password"
                        help="Confirm your new password"
                        validation="required|confirmed:password"
                        validation-visibility="live"
                        outer-class="w-[60%] flex flex-col justify-center items-center"
                        wrapper-class="w-full flex flex-col justify-center"
                        label-class="block mb-1 font-bold text-sm text-gray-700"
                        inner-class="border-gray-400 rounded-lg mb-2 overflow-hidden focus:outline-none m-1"
                        input-class="w-full h-10 px-3 text-base text-gray-700 placeholder-gray-400 focus:outline-none"
                        help-class="w-[70%] text-xs text-gray-500"
                        messages-class="w-[70%] text-xs text-gray-500"
                    />
                </FormKit>
                <FormKit
                    type="submit"
                    label="Login"
                    @click="login"
                    outer-class="w-[45%] flex flex-col justify-center items-end"
                    wrapper-class="flex flex-col justify-center"
                />
            </div>

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
// import logo from '@/assets/logo.png';
import Img from '@/assets/loginImg.jpg';

export default defineComponent({
    name: 'Login',
    data() {
        return {
            requestModel: {
                email: '',
                password: ''
            },
            img: Img
        };
    },
    methods: {
        async login() {
            try {
                const response = await api.post('/user/login', this.requestModel, { withCredentials: true });
                console.log(response.data as LoginResponse);
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
main {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
    height: calc(100vh - 70px);
    width: 100vw;
    background-color: var(--secundary-color);
}

.box {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 98%;
    width: 48%;
    border-radius: 10px;
}

.box img {
    height: 100%;
    width: 100%;
    border-radius: 10px;
    object-fit: cover;
}

.box h2{
    color: var(--primary-color);
    font-size: 2rem;
}

[data-invalid] .formkit-inner {
  border-color: red;
  box-shadow: 0 0 0 1px red;
}

[data-complete] .formkit-inner {
  border-color: red;
  box-shadow: 0 0 0 1px green;
}
[data-complete] .formkit-inner::after {
  content: '✅';
  display: block;
  padding: 0.5em;
}

@media screen and (max-width: 768px) {
    main {
        flex-direction: column;
    }
    .box {
        width: 100%;
    }

    #box1 {
        display: none;
    }
    
}

</style>