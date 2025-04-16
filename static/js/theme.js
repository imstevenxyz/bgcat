"use strict";

/*
 * Toggle between all themes
 */
function toggleTheme() {
  if (localStorage.getItem("theme") === "theme-auto") {
    setTheme("theme-light");
  } else if (localStorage.getItem("theme") === "theme-light") {
    setTheme("theme-dark");
  } else {
    setTheme("theme-auto");
  }
}

/*
 * Set a theme
 */
function setTheme(theme) {
  document.documentElement.className = theme;
  localStorage.setItem("theme", theme);
  if (localStorage.getItem("theme") === "theme-light") {
    document.getElementById("theme-icon").classList = "fas fa-circle fa-fw";
  } else if (localStorage.getItem("theme") === "theme-dark") {
    document.getElementById("theme-icon").classList = "far fa-circle fa-fw";
  } else {
    document.getElementById("theme-icon").classList = "fas fa-adjust fa-fw";
  }
  console.log("bgcat: set " + theme);
}

/*
 * Set the theme on initialization if it is set
 */
(function () {
  if (localStorage.getItem("theme") === null) {
    setTheme("theme-auto");
  } else {
    setTheme(localStorage.getItem("theme"));
  }
})();
