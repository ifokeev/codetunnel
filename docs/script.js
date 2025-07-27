// Smooth scrolling for navigation links
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

// Add scroll effect to header
const header = document.querySelector('.header');
let lastScroll = 0;

window.addEventListener('scroll', () => {
    const currentScroll = window.pageYOffset;
    
    if (currentScroll > 0) {
        header.style.background = 'rgba(10, 10, 10, 0.95)';
        header.style.boxShadow = '0 2px 20px rgba(0, 0, 0, 0.5)';
    } else {
        header.style.background = 'rgba(10, 10, 10, 0.8)';
        header.style.boxShadow = 'none';
    }
    
    lastScroll = currentScroll;
});

// Intersection Observer for animations
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Observe elements
document.querySelectorAll('.feature, .step, .download-card').forEach(el => {
    el.style.opacity = '0';
    el.style.transform = 'translateY(20px)';
    el.style.transition = 'opacity 0.6s ease-out, transform 0.6s ease-out';
    observer.observe(el);
});

// Terminal typing effect
const terminalLines = document.querySelectorAll('.terminal-line');
terminalLines.forEach((line, index) => {
    line.style.opacity = '0';
    setTimeout(() => {
        line.style.transition = 'opacity 0.3s ease-in';
        line.style.opacity = '1';
    }, index * 500);
});

// Copy to clipboard functionality (for future use)
function copyToClipboard(text) {
    navigator.clipboard.writeText(text).then(() => {
        console.log('Copied to clipboard');
    }).catch(err => {
        console.error('Failed to copy:', err);
    });
}

// Update download links with latest version
async function updateDownloadLinks() {
    try {
        const response = await fetch('https://api.github.com/repos/ifokeev/codetunnel/releases/latest');
        const data = await response.json();
        const version = data.tag_name || 'v1.0.0';
        
        // Remove 'v' prefix if present
        const versionNumber = version.replace('v', '');
        
        // Update download links with actual version
        document.querySelectorAll('.download-card').forEach(card => {
            const href = card.getAttribute('href');
            if (href && href.includes('x.x.x')) {
                const newHref = href.replace('x.x.x', versionNumber);
                card.setAttribute('href', newHref);
                console.log('Updated download link:', newHref);
            }
        });
        
        // Also update any visible version text
        document.querySelectorAll('.version-text').forEach(el => {
            el.textContent = version;
        });
    } catch (error) {
        console.error('Failed to fetch latest version:', error);
        // Fallback to a default version
        document.querySelectorAll('.download-card').forEach(card => {
            const href = card.getAttribute('href');
            if (href && href.includes('x.x.x')) {
                card.setAttribute('href', href.replace('x.x.x', '1.0.0'));
            }
        });
    }
}

// Initialize on load
document.addEventListener('DOMContentLoaded', () => {
    updateDownloadLinks();
});