<template>
  <div class="nav w-full">
    <router-link to="/numbers">Nombres</router-link>
    <router-link to="/charts">Graphes</router-link>
  </div>
</template>

<script setup lang="ts"></script>
<style scoped>
.nav {
  display: flex;
  justify-content: space-around;
  align-items: center;
  background-color: #333;
  padding: 1rem;
}

.nav a {
  color: white;
  text-decoration: none;
  font-weight: bold;
}

.nav a:hover {
  text-decoration: underline;
}
</style>
