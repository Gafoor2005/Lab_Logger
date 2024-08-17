const menuButton = document.getElementById('menu-button');
const dropdownMenu = document.getElementById('switch-db-dropdown');

menuButton.addEventListener('click', () => {
  dropdownMenu.classList.toggle('hidden');
});

document.addEventListener('click', (event) => {
    if (!dropdownMenu.contains(event.target) && !menuButton.contains(event.target)) {
      dropdownMenu.classList.add('hidden');
    }
});
