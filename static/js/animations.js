// Custom Cursor (disabled for better performance)
// Uncomment below to enable custom cursor
/*
const cursor = document.createElement('div');
const cursorFollower = document.createElement('div');
cursor.className = 'cursor';
cursorFollower.className = 'cursor-follower';
document.body.appendChild(cursor);
document.body.appendChild(cursorFollower);

let mouseX = 0, mouseY = 0;
let followerX = 0, followerY = 0;

document.addEventListener('mousemove', (e) => {
    mouseX = e.clientX;
    mouseY = e.clientY;
    cursor.style.transform = `translate(${mouseX}px, ${mouseY}px) translate(-50%, -50%)`;
});

function animateFollower() {
    followerX += (mouseX - followerX) * 0.15;
    followerY += (mouseY - followerY) * 0.15;
    cursorFollower.style.transform = `translate(${followerX}px, ${followerY}px) translate(-50%, -50%)`;
    requestAnimationFrame(animateFollower);
}
animateFollower();
*/

// Animated Capsules Background
function createAnimatedCapsules() {
    const particles = document.getElementById('particles');
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    canvas.style.position = 'absolute';
    canvas.style.top = '0';
    canvas.style.left = '0';
    particles.appendChild(canvas);
    
    class Capsule {
        constructor() {
            this.x = Math.random() * canvas.width;
            this.y = Math.random() * canvas.height;
            this.width = Math.random() * 80 + 60;
            this.height = 25;
            this.rotation = Math.random() * Math.PI / 4 - Math.PI / 8; // Slight angle variation
            this.speedX = 0.3 + Math.random() * 0.2; // Move right
            this.speedY = 0.1 + Math.random() * 0.1; // Slight downward drift
            this.opacity = Math.random() * 0.4 + 0.2;
            this.colorIndex = Math.floor(Math.random() * 4);
        }
        
        getGradient() {
            const gradients = [
                ['#E89A7C', '#D67B5C'],
                ['#F5B89A', '#E89A7C'],
                ['#D67B5C', '#C96A4A'],
                ['#E89A7C', '#F5B89A']
            ];
            return gradients[this.colorIndex];
        }
        
        update() {
            this.x += this.speedX;
            this.y += this.speedY;
            this.rotation += 0.001;
            
            // Wrap around screen
            if (this.x > canvas.width + this.width) this.x = -this.width;
            if (this.y > canvas.height + this.height) this.y = -this.height;
        }
        
        draw() {
            ctx.save();
            ctx.translate(this.x, this.y);
            ctx.rotate(this.rotation);
            
            // Create gradient
            const gradient = ctx.createLinearGradient(-this.width/2, 0, this.width/2, 0);
            const colors = this.getGradient();
            gradient.addColorStop(0, colors[0] + Math.floor(this.opacity * 255).toString(16).padStart(2, '0'));
            gradient.addColorStop(1, colors[1] + Math.floor(this.opacity * 255).toString(16).padStart(2, '0'));
            
            // Draw capsule with gradient fill
            ctx.fillStyle = gradient;
            ctx.beginPath();
            
            // Left semicircle
            ctx.arc(-this.width/2 + this.height/2, 0, this.height/2, Math.PI/2, Math.PI * 1.5);
            // Top line
            ctx.lineTo(this.width/2 - this.height/2, -this.height/2);
            // Right semicircle
            ctx.arc(this.width/2 - this.height/2, 0, this.height/2, Math.PI * 1.5, Math.PI/2);
            // Bottom line
            ctx.lineTo(-this.width/2 + this.height/2, this.height/2);
            
            ctx.closePath();
            ctx.fill();
            
            // Add subtle outline
            ctx.strokeStyle = colors[1] + Math.floor(this.opacity * 0.5 * 255).toString(16).padStart(2, '0');
            ctx.lineWidth = 1;
            ctx.stroke();
            
            ctx.restore();
        }
    }
    
    const capsules = [];
    for (let i = 0; i < 20; i++) {
        capsules.push(new Capsule());
    }
    
    function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        
        capsules.forEach(capsule => {
            capsule.update();
            capsule.draw();
        });
        
        requestAnimationFrame(animate);
    }
    
    animate();
    
    window.addEventListener('resize', () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    });
}

// Smooth scroll
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    });
});

// Intersection Observer for animations
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -100px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Observe all cards
document.addEventListener('DOMContentLoaded', () => {
    createAnimatedCapsules();
    
    const cards = document.querySelectorAll('.feature-card, .command-card');
    cards.forEach((card, index) => {
        card.style.opacity = '0';
        card.style.transform = 'translateY(30px)';
        card.style.transition = `all 0.6s ease ${index * 0.1}s`;
        observer.observe(card);
    });
});

// Navbar scroll effect
let lastScroll = 0;
window.addEventListener('scroll', () => {
    const navbar = document.querySelector('.navbar');
    const currentScroll = window.pageYOffset;
    
    if (currentScroll > 100) {
        navbar.style.background = 'rgba(17, 24, 39, 0.95)';
        navbar.style.boxShadow = '0 4px 20px rgba(0, 0, 0, 0.3)';
    } else {
        navbar.style.background = 'rgba(17, 24, 39, 0.8)';
        navbar.style.boxShadow = 'none';
    }
    
    lastScroll = currentScroll;
});

// Button ripple effect
document.querySelectorAll('.btn-primary, .btn-secondary, .btn-cta').forEach(button => {
    button.addEventListener('click', function(e) {
        const ripple = document.createElement('span');
        const rect = this.getBoundingClientRect();
        const size = Math.max(rect.width, rect.height);
        const x = e.clientX - rect.left - size / 2;
        const y = e.clientY - rect.top - size / 2;
        
        ripple.style.cssText = `
            position: absolute;
            width: ${size}px;
            height: ${size}px;
            border-radius: 50%;
            background: rgba(255, 255, 255, 0.5);
            left: ${x}px;
            top: ${y}px;
            transform: scale(0);
            animation: ripple 0.6s ease-out;
            pointer-events: none;
        `;
        
        this.appendChild(ripple);
        setTimeout(() => ripple.remove(), 600);
    });
});

// Add ripple animation
const style = document.createElement('style');
style.textContent = `
    @keyframes ripple {
        to {
            transform: scale(4);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);
