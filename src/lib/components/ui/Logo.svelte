<script lang="ts">
  export let size = 40;
  export let sizeY: number | undefined = undefined;
  export let sizeX: number | undefined = undefined;
  let className = "";
  export { className as class };
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://w3.org"
  xml:space="preserve"
  class={className}
  height={sizeY ?? size}
  width={sizeX ?? size}
  viewBox="0 0 135.47 135.47"
  ><defs
    ><linearGradient id="linearGradient2"
      ><stop offset="0" stop-color="none" stop-opacity="0" /><stop
        offset=".814"
        stop-color="none"
      /></linearGradient
    ><linearGradient
      xlink:href="#linearGradient2"
      id="linearGradient37"
      x1="5.349"
      x2="78.32"
      y1="65.233"
      y2="65.233"
      gradientTransform="matrix(1.0399 0 0 1.1365 .015 -6.172)"
      gradientUnits="userSpaceOnUse"
    /></defs
  ><ellipse
    cx="72.465"
    cy="67.968"
    fill="none"
    stroke="currentColor"
    stroke-linecap="round"
    stroke-width="10.426"
    rx="53.465"
    ry="54.194"
    style="paint-order:fill markers stroke"
  /><g stroke="currentColor" stroke-linecap="round" stroke-width="2.592"
    >
    <!-- ----------------------HINGES----------------- -->
    <rect
      width="8.84"
      height="33.124"
      x="5.817"
      y="87.59"
      fill="currentColor"
      ry="1.725"
      style="paint-order:fill markers stroke"
    /><rect
      width="8.84"
      height="33.124"
      x="5.817"
      y="16.1"
      fill="currentColor"
      ry="1.725"
      style="paint-order:fill markers stroke"
    />
    <!-- ---------------------------------------------- -->
    <path
      fill="currentColor"
      d="m41.271 13.547-.021.001c-1.22.006-2.438.518-3.372 1.539l-8.062 8.81h-23v17.115h19.195v54.377H6.817v17.114H26.01v0h3.901l7.965 8.706c.935 1.021 2.153 1.531 3.373 1.537l.02.002v0h29.715V109.67H53.64a13.545 14.803 0 0 1-9.577-4.337l-6.433-7.03v-60.31l6.433-7.031a13.545 14.803 0 0 1 9.577-4.335h17.346V13.55H41.3l-.03-.002z"
      style="paint-order:fill markers stroke"
    /><rect
      width="11.401"
      height="26.706"
      x="68.822"
      y="2.16"
      fill="currentColor"
      ry="3.519"
      style="paint-order:fill markers stroke"
    /><rect
      width="11.401"
      height="26.706"
      x="68.822"
      y="107.07"
      fill="currentColor"
      ry="3.519"
      style="paint-order:fill markers stroke"
    /></g
  ><g fill="currentColor" class="i-rudder"
    ><path
      d="M72.464 32.592c-19.284 0-34.94 15.896-34.94 35.375s15.656 35.377 34.94 35.377 34.943-15.899 34.943-35.377-15.659-35.375-34.943-35.375m0 6.208c15.883 0 28.735 12.999 28.735 29.167s-12.852 29.17-28.735 29.17-28.733-13.002-28.733-29.17S56.581 38.8 72.464 38.8"
      style="paint-order:fill markers stroke"
    /><path
      fill-rule="evenodd"
      d="M41.947 37.45a3.727 3.727 0 0 0 .006 5.282l20.83 20.83 5.277-5.275-20.831-20.831a3.727 3.727 0 0 0-5.283-.006zm34.859 40.134 20.895 20.895a3.726 3.726 0 0 0 5.282.006 3.727 3.727 0 0 0-.006-5.282L82.082 72.308zM102.98 37.45a3.727 3.727 0 0 0-5.282.006l-20.83 20.83 5.275 5.277 20.831-20.831a3.727 3.727 0 0 0 .006-5.283zM62.846 72.309 41.951 93.205a3.727 3.727 0 0 0-.006 5.282 3.727 3.727 0 0 0 5.282-.006l20.896-20.895z"
      style="paint-order:fill markers stroke"
    /></g
  ><circle
    cx="72.465"
    cy="67.968"
    r="13.238"
    fill="none"
    stroke="currentColor"
    stroke-linecap="round"
    stroke-width="7.388"
    style="paint-order:fill markers stroke"
  /></svg
>

<style>
  .i-rudder {
    animation: spin 5s ease-in-out;
    transform-box: fill-box;
    transform-origin: center;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    20% {
      transform: rotate(12deg);
    }
    30% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>