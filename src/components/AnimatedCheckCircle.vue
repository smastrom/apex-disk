<!--
AnimatedCheckCircle

Purpose: Animated SVG check-circle icon. The circle scales in (pop), then the checkmark draws itself.

Props: size (number, default 48)

Example:
 <AnimatedCheckCircle :size="48" />
-->

<script setup lang="ts">
withDefaults(defineProps<{ size?: number }>(), { size: 48 })
</script>

<template>
   <svg
      class="AnimatedCheckCircle-root"
      :width="size"
      :height="size"
      viewBox="0 0 256 256"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
   >
      <circle
         class="AnimatedCheckCircle-circle"
         cx="128"
         cy="128"
         r="120"
         fill="currentColor"
      />
      <path
         class="AnimatedCheckCircle-check"
         d="M85 130 L115 160 L175 100"
         stroke="var(--color-bg, #fff)"
         stroke-width="18"
         stroke-linecap="round"
         stroke-linejoin="round"
         fill="none"
      />
   </svg>
</template>

<style scoped>
.AnimatedCheckCircle-root {
   overflow: visible;
}

.AnimatedCheckCircle-circle {
   transform-origin: center;
   animation: AnimatedCheckCircle-pop 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) both;
}

.AnimatedCheckCircle-check {
   stroke-dasharray: 160;
   stroke-dashoffset: 160;
   animation: AnimatedCheckCircle-draw 0.35s ease-out 0.3s forwards;
}

@keyframes AnimatedCheckCircle-pop {
   0% {
      transform: scale(0);
      opacity: 0;
   }
   100% {
      transform: scale(1);
      opacity: 1;
   }
}

@keyframes AnimatedCheckCircle-draw {
   to {
      stroke-dashoffset: 0;
   }
}
</style>
