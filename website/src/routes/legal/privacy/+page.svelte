<script lang="ts">
	import { Separator } from '$lib/components/ui/separator';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import ShieldCheck from 'lucide-svelte/icons/shield-check';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	const LAST_UPDATED = 'April 20, 2025';
	const CONTACT_EMAIL = 'contact@outpoot.com';
</script>

<svelte:head>
	<title>Privacy Policy: Vyntr</title>
	<meta
		name="description"
		content="Learn how Vyntr protects your privacy while searching the web."
	/>
	<meta name="keywords" content="privacy, policy, data protection, vyntr, search engine" />
	<meta property="og:title" content="Privacy Policy: Vyntr" />
	<meta
		property="og:description"
		content="Learn how Vyntr protects your privacy while searching the web."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:url" content={page.url.href} />
</svelte:head>

<div class="container mx-auto max-w-4xl py-10">
	<Card.Root class="p-6">
		<div class="mb-8 flex items-center gap-3">
			<ShieldCheck class="h-10 w-10 text-primary" />
			<div>
				<h1 class="text-4xl font-bold">Privacy Policy</h1>
				<p class="text-muted-foreground">
					Last updated: {LAST_UPDATED}
				</p>
			</div>
		</div>

		 <main class="main">
            <div class="hero">
                <h1 class="hero-title">Privacy Policy</h1>
                <p class="hero-subtitle">Last updated: <span class="date">December 2024</span></p>
                <div class="hero-decoration"></div>
            </div>

            <div class="content">
                <div class="toc">
                    <h3>Quick Navigation</h3>
                    <ul class="toc-list">
                        <li><a href="#overview" class="toc-link">Overview</a></li>
                        <li><a href="#collection" class="toc-link">Information We Collect</a></li>
                        <li><a href="#usage" class="toc-link">How We Use Information</a></li>
                        <li><a href="#sharing" class="toc-link">Information Sharing</a></li>
                        <li><a href="#security" class="toc-link">Data Security</a></li>
                        <li><a href="#rights" class="toc-link">Your Rights</a></li>
                        <li><a href="#cookies" class="toc-link">Cookies & Tracking</a></li>
                        <li><a href="#changes" class="toc-link">Policy Changes</a></li>
                        <li><a href="#contact" class="toc-link">Contact Us</a></li>
                    </ul>
                </div>

                <div class="policy-content">
                    <section id="overview" class="section">
                        <h2 class="section-title">Privacy Overview</h2>
                        <div class="highlight-box">
                            <p>At BROWSIFY, we believe privacy is a fundamental right. We've built our search engine with privacy by design, ensuring your searches remain anonymous and your data stays protected.</p>
                        </div>
                        <p>This Privacy Policy explains how we collect, use, and protect your information when you use our search engine service. We are committed to transparency and giving you control over your data.</p>
                    </section>

                    <section id="collection" class="section">
                        <h2 class="section-title">Information We Collect</h2>
                        
                        <div class="info-grid">
                            <div class="info-card">
                                <div class="info-icon">🔍</div>
                                <h3>Search Queries</h3>
                                <p>We process your search terms to provide relevant results, but we don't store them linked to your identity.</p>
                            </div>
                            <div class="info-card">
                                <div class="info-icon">📊</div>
                                <h3>Technical Data</h3>
                                <p>Basic technical information like browser type and general location (country-level) for service optimization.</p>
                            </div>
                            <div class="info-card">
                                <div class="info-icon">🛡️</div>
                                <h3>No Personal Data</h3>
                                <p>We don't collect names, email addresses, or create user profiles. Your searches are anonymous.</p>
                            </div>
                        </div>
                    </section>

                    <section id="usage" class="section">
                        <h2 class="section-title">How We Use Information</h2>
                        <ul class="usage-list">
                            <li><span class="usage-icon">✅</span> Provide search results and improve search quality</li>
                            <li><span class="usage-icon">✅</span> Detect and prevent spam, abuse, and security threats</li>
                            <li><span class="usage-icon">✅</span> Analyze usage patterns in aggregate to improve our service</li>
                            <li><span class="usage-icon">✅</span> Ensure our systems are working properly</li>
                            <li><span class="usage-icon">❌</span> Create user profiles or behavioral tracking</li>
                            <li><span class="usage-icon">❌</span> Target advertising based on your searches</li>
                            <li><span class="usage-icon">❌</span> Share your data with third parties for marketing</li>
                        </ul>
                    </section>

                    <section id="sharing" class="section">
                        <h2 class="section-title">Information Sharing</h2>
                        <div class="sharing-policy">
                            <div class="policy-statement">
                                <h3>We Do Not Sell Your Data</h3>
                                <p>BROWSIFY will never sell, rent, or trade your personal information to third parties.</p>
                            </div>
                            <div class="limited-sharing">
                                <h4>Limited Sharing Only When:</h4>
                                <ul>
                                    <li>Required by law or legal process</li>
                                    <li>Necessary to protect our rights or property</li>
                                    <li>With service providers who help operate our service (under strict privacy agreements)</li>
                                </ul>
                            </div>
                        </div>
                    </section>

                    <section id="security" class="section">
                        <h2 class="section-title">Data Security</h2>
                        <div class="security-features">
                            <div class="security-item">
                                <div class="security-badge">🔒</div>
                                <div>
                                    <h4>HTTPS Encryption</h4>
                                    <p>All communications are encrypted in transit</p>
                                </div>
                            </div>
                            <div class="security-item">
                                <div class="security-badge">🗂️</div>
                                <div>
                                    <h4>Minimal Data Storage</h4>
                                    <p>We store only what's necessary and delete it regularly</p>
                                </div>
                            </div>
                            <div class="security-item">
                                <div class="security-badge">🛡️</div>
                                <div>
                                    <h4>Security Audits</h4>
                                    <p>Regular security reviews and updates</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section id="rights" class="section">
                        <h2 class="section-title">Your Privacy Rights</h2>
                        <div class="rights-grid">
                            <div class="right-item">
                                <h4>Right to Access</h4>
                                <p>Request information about what data we have (spoiler: very little!)</p>
                            </div>
                            <div class="right-item">
                                <h4>Right to Delete</h4>
                                <p>Request deletion of any data we may have collected</p>
                            </div>
                            <div class="right-item">
                                <h4>Right to Opt-Out</h4>
                                <p>Disable any optional data collection features</p>
                            </div>
                            <div class="right-item">
                                <h4>Right to Portability</h4>
                                <p>Export your data in a standard format (where applicable)</p>
                            </div>
                        </div>
                    </section>

                    <section id="cookies" class="section">
                        <h2 class="section-title">Cookies & Tracking</h2>
                        <div class="cookie-policy">
                            <div class="cookie-type">
                                <h4>Essential Cookies Only</h4>
                                <p>We use minimal, essential cookies to make our service work properly. These include:</p>
                                <ul>
                                    <li>Session management</li>
                                    <li>Security features</li>
                                    <li>Basic preferences</li>
                                </ul>
                            </div>
                            <div class="no-tracking">
                                <h4>No Tracking Cookies</h4>
                                <p>We don't use advertising cookies, analytics cookies that track you across sites, or social media tracking pixels.</p>
                            </div>
                        </div>
                    </section>

                    <section id="changes" class="section">
                        <h2 class="section-title">Policy Changes</h2>
                        <p>We may update this Privacy Policy occasionally to reflect changes in our practices or for legal reasons. When we do:</p>
                        <ul>
                            <li>We'll post the updated policy on this page</li>
                            <li>We'll update the "Last updated" date at the top</li>
                            <li>For significant changes, we'll provide notice on our homepage</li>
                        </ul>
                        <p>We encourage you to review this policy periodically to stay informed about how we protect your privacy.</p>
                    </section>

                    <section id="contact" class="section">
                        <h2 class="section-title">Contact Us</h2>
                        <div class="contact-info">
                            <p>If you have questions about this Privacy Policy or want to exercise your privacy rights, please contact us:</p>
                            <div class="contact-methods">
                                <div class="contact-method">
                                    <span class="contact-icon">📧</span>
                                    <div>
                                        <strong>Email:</strong>
                                        <a href="mailto:privacy@browsify.com">privacy@browsify.com</a>
                                    </div>
                                </div>
                                <div class="contact-method">
                                    <span class="contact-icon">🌐</span>
                                    <div>
                                        <strong>Web:</strong>
                                        <a href="#">browsify.com/contact</a>
                                    </div>
                                </div>
                            </div>
                            <p class="response-time">We typically respond to privacy inquiries within 48 hours.</p>
                        </div>
                    </section>
                </div>
            </div>
        </main>

			<div class="text-sm text-muted-foreground">
				<p>Last Updated: {LAST_UPDATED}</p>
				<p>Contact: <a href="mailto:{CONTACT_EMAIL}">{CONTACT_EMAIL}</a></p>
			</div>
		</div>

		<Separator class="my-8" />

		<div class="flex justify-center">
			<Button variant="outline" size="lg" onclick={() => goto('/legal/terms')}>
				Read our Terms of Service →
			</Button>
		</div>
	</Card.Root>
</div>
