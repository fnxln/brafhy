import axios from 'axios';
import router from '@/router';

// quero usar um redirecionamento para a página dashboard após o login

const api = axios.create({
  baseURL: 'http://localhost:3000', // Insira a URL base do seu backend aqui
});

export default api;