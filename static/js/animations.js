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

// Premium Animated Background
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
    
    // Floating orbs with glow
    class Orb {
        constructor() {
            this.x = Math.random() * canvas.width;
            this.y = Math.random() * canvas.height;
            this.radius = Math.random() * 3 + 2;
            this.speedX = (Math.random() - 0.5) * 0.5;
            this.speedY = (Math.random() - 0.5) * 0.5;
            this.hue = Math.random() * 30 + 10; // Orange/coral hues
        }
        
        update() {
            this.x += this.speedX;
            this.y += this.speedY;
            
            if (this.x < 0 || this.x > canvas.width) this.speedX *= -1;
            if (this.y < 0 || this.y > canvas.height) this.speedY *= -1;
        }
        
        draw() {
            // Glow effect
            const gradient = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, this.radius * 3);
            gradient.addColorStop(0, `hsla(${this.hue}, 80%, 65%, 0.8)`);
            gradient.addColorStop(0.5, `hsla(${this.hue}, 80%, 65%, 0.3)`);
            gradient.addColorStop(1, `hsla(${this.hue}, 80%, 65%, 0)`);
            
            ctx.fillStyle = gradient;
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.radius * 3, 0, Math.PI * 2);
            ctx.fill();
            
            // Core
            ctx.fillStyle = `hsla(${this.hue}, 90%, 70%, 0.9)`;
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
            ctx.fill();
        }
    }
    
    // Flowing waves
    class Wave {
        constructor(y, speed, amplitude, frequency) {
            this.y = y;
            this.speed = speed;
            this.amplitude = amplitude;
            this.frequency = frequency;
            this.offset = Math.random() * Math.PI * 2;
        }
        
        draw(time) {
            ctx.beginPath();
            ctx.moveTo(0, this.y);
            
            for (let x = 0; x <= canvas.width; x += 5) {
                const y = this.y + Math.sin((x * this.frequency) + (time * this.speed) + this.offset) * this.amplitude;
                ctx.lineTo(x, y);
            }
            
            const gradient = ctx.createLinearGradient(0, this.y - this.amplitude, 0, this.y + this.amplitude);
            gradient.addColorStop(0, 'rgba(232, 154, 124, 0)');
            gradient.addColorStop(0.5, 'rgba(232, 154, 124, 0.1)');
            gradient.addColorStop(1, 'rgba(232, 154, 124, 0)');
            
            ctx.strokeStyle = gradient;
            ctx.lineWidth = 2;
            ctx.stroke();
        }
    }
    
    // Geometric shapes
    class GeometricShape {
        constructor() {
            this.x = Math.random() * canvas.width;
            this.y = Math.random() * canvas.height;
            this.size = Math.random() * 60 + 40;
            this.rotation = Math.random() * Math.PI * 2;
            this.rotationSpeed = (Math.random() - 0.5) * 0.02;
            this.speedX = (Math.random() - 0.5) * 0.3;
            this.speedY = (Math.random() - 0.5) * 0.3;
            this.type = Math.floor(Math.random() * 3); // 0: hexagon, 1: triangle, 2: diamond
            this.opacity = Math.random() * 0.15 + 0.05;
        }
        
        update() {
            this.x += this.speedX;
            this.y += this.speedY;
            this.rotation += this.rotationSpeed;
            
            if (this.x < -this.size) this.x = canvas.width + this.size;
            if (this.x > canvas.width + this.size) this.x = -this.size;
            if (this.y < -this.size) this.y = canvas.height + this.size;
            if (this.y > canvas.height + this.size) this.y = -this.size;
        }
        
        draw() {
            ctx.save();
            ctx.translate(this.x, this.y);
            ctx.rotate(this.rotation);
            
            // Gradient fill
            const gradient = ctx.createLinearGradient(-this.size/2, -this.size/2, this.size/2, this.size/2);
            gradient.addColorStop(0, `rgba(232, 154, 124, ${this.opacity})`);
            gradient.addColorStop(1, `rgba(214, 123, 92, ${this.opacity * 0.5})`);
            
            ctx.fillStyle = gradient;
            ctx.strokeStyle = `rgba(232, 154, 124, ${this.opacity * 2})`;
            ctx.lineWidth = 1;
            
            ctx.beginPath();
            
            if (this.type === 0) {
                // Hexagon
                for (let i = 0; i < 6; i++) {
                    const angle = (Math.PI / 3) * i;
                    const x = Math.cos(angle) * this.size / 2;
                    const y = Math.sin(angle) * this.size / 2;
                    if (i === 0) ctx.moveTo(x, y);
                    else ctx.lineTo(x, y);
                }
            } else if (this.type === 1) {
                // Triangle
                for (let i = 0; i < 3; i++) {
                    const angle = (Math.PI * 2 / 3) * i - Math.PI / 2;
                    const x = Math.cos(angle) * this.size / 2;
                    const y = Math.sin(angle) * this.size / 2;
                    if (i === 0) ctx.moveTo(x, y);
                    else ctx.lineTo(x, y);
                }
            } else {
                // Diamond
                ctx.moveTo(0, -this.size / 2);
                ctx.lineTo(this.size / 2, 0);
                ctx.lineTo(0, this.size / 2);
                ctx.lineTo(-this.size / 2, 0);
            }
            
            ctx.closePath();
            ctx.fill();
            ctx.stroke();
            
            ctx.restore();
        }
    }
    
    // Create elements
    const orbs = [];
    for (let i = 0; i < 30; i++) {
        orbs.push(new Orb());
    }
    
    const waves = [
        new Wave(canvas.height * 0.3, 0.001, 30, 0.005),
        new Wave(canvas.height * 0.5, 0.0015, 40, 0.003),
        new Wave(canvas.height * 0.7, 0.002, 25, 0.007)
    ];
    
    const shapes = [];
    for (let i = 0; i < 8; i++) {
        shapes.push(new GeometricShape());
    }
    
    let time = 0;
    
    function animate() {
        // Fade effect for trails
        ctx.fillStyle = 'rgba(10, 10, 15, 0.1)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        time += 0.01;
        
        // Draw waves
        waves.forEach(wave => wave.draw(time));
        
        // Draw and update shapes
        shapes.forEach(shape => {
            shape.update();
            shape.draw();
        });
        
        // Draw and update orbs
        orbs.forEach(orb => {
            orb.update();
            orb.draw();
        });
        
        // Connect nearby orbs
        for (let i = 0; i < orbs.length; i++) {
            for (let j = i + 1; j < orbs.length; j++) {
                const dx = orbs[i].x - orbs[j].x;
                const dy = orbs[i].y - orbs[j].y;
                const distance = Math.sqrt(dx * dx + dy * dy);
                
                if (distance < 150) {
                    ctx.strokeStyle = `rgba(232, 154, 124, ${0.2 * (1 - distance / 150)})`;
                    ctx.lineWidth = 1;
                    ctx.beginPath();
                    ctx.moveTo(orbs[i].x, orbs[i].y);
                    ctx.lineTo(orbs[j].x, orbs[j].y);
                    ctx.stroke();
                }
            }
        }
        
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
