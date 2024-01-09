<script setup lang="ts">
import { io } from 'socket.io-client';
const socket = io('http://localhost:3000');
socket.on('connect', () => {
  console.log('connected');
});

</script>

<template>
  <div class="button">
    <button @click="socket.emit('message', 'Hello world!')">Send</button>
  </div>
</template> 

<style scoped></style>
