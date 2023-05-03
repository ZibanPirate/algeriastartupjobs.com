use crate::{
  _utils::fuzzy_word_exists::fuzzy_word_exists,
  account::{
    mocks::generate_accounts_seed,
    model::{Account, AccountTrait, AccountType},
  },
  category::mocks::generate_categories_seed,
  tag::{mocks::generate_tags_seed, model::Tag},
};

use super::model::{PartialPost, PartialPostTrait, Post};
use titlecase::titlecase;

pub fn generate_one_post_mock(post_id: i32) -> Post {
  Post {
    id: post_id,
    slug: format!("post_{}", post_id),
    title: format!("Post {}", post_id),
    poster_id: post_id,
    short_description: format!("Short description for post {}", post_id),
    description: format!("Description for post {}", post_id),
    category_id: post_id,
    tag_ids: vec![post_id],
  }
}

pub fn generate_many_post_mocks_with_overwrite<F>(
  from: i32,
  to: i32,
  overwrite: Option<F>,
) -> Vec<Post>
where
  F: Fn(i32) -> PartialPost,
{
  let mut posts: Vec<Post> = Vec::new();
  for i in from..to {
    let post = match overwrite {
      Some(ref f) => {
        let partial_post = f(i);
        let default_post = generate_one_post_mock(i);
        partial_post.to_post(default_post)
      }
      None => generate_one_post_mock(i),
    };
    posts.push(post);
  }
  posts
}

pub fn generate_many_post_mocks(from: i32, to: i32) -> Vec<Post> {
  generate_many_post_mocks_with_overwrite(
    from,
    to,
    Some(|_id| PartialPost {
      id: None,
      slug: None,
      title: None,
      poster_id: None,
      short_description: None,
      description: None,
      category_id: None,
      tag_ids: None,
    }),
  )
}

struct PostSeed {
  pub title: String,
  pub description: String,
}

pub fn generate_posts_seed() -> Vec<Post> {
  let jobs = [
        PostSeed {
            title: "Senior Fullstack JavaScript Developer".to_string(),
            description: r#"Yassiron is a leading company in the field of hybrid solutions using C++, C#, and JavaScript. We are looking for a Senior Fullstack JavaScript Developer to join our team and work on developing, designing, and testing web portals that are fast and responsive.

As a Senior Fullstack JavaScript Developer, you will be responsible for:

Developing solutions using C++, C#, and JavaScript
Contributing to development, design and test automation best practices and procedures
Working with 3rd party engineering team to understand the current technology, processes, and procedures and onboard those processes internally
Building next-generation web portals that are fast and responsive
Working with UX and UI teams to provide users with the best possible experience
To be successful in this role, you should have:

Bachelor’s degree in Computer Science or related field
At least 5 years of experience in full-stack development using JavaScript frameworks such as Angular, React, Vue, and Node.js
Proficiency in C++, C#, and JavaScript
Experience with web development tools such as HTML, CSS, Bootstrap, jQuery, etc.
Knowledge of RESTful APIs, JSON, XML, etc.
Familiarity with Git, CI/CD tools, cloud platforms, etc.
Excellent communication, collaboration, and problem-solving skills
If you are interested in this position, please apply with your resume and portfolio. We offer a competitive salary, benefits, and a flexible work environment. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Junior Rust Developer".to_string(),
            description: r#"Specific Emballage is a company that specializes in creating custom packaging solutions for various industries. We are looking for a Junior Rust Developer to join our team and work on developing software programs written in the Rust programming language.

As a Junior Rust Developer, you will be responsible for:

Developing software in Rust
Maintaining and improving existing Rust codebases
Collaborating with other developers to create high-quality software
Troubleshooting and resolving Rust bugs
To be successful in this role, you should have:

Experience with Rust programming language
Experience with object-oriented programming
Knowledge of web development tools such as HTML, CSS, JavaScript, etc.
Familiarity with Git, CI/CD tools, cloud platforms, etc.
Good communication, collaboration, and problem-solving skills
If you are interested in this position, please apply with your resume and portfolio. We offer a competitive salary, benefits, and a friendly work environment. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Mid Backend Engineer Java".to_string(),
            description: r#"Startdown.dz is a company that provides innovative solutions for startups in Algeria. We are looking for a Mid Backend Engineer Java to join our team and work on developing and maintaining web applications using Java and other technologies.

As a Mid Backend Engineer Java, you will be responsible for:

Developing web applications using Java and other frameworks
Implementing the backend logic and functionality of the applications
Collaborating with the front-end team to ensure compatibility and usability
Testing and debugging the applications
Documenting the designs, code and project assets
To be successful in this role, you should have:

Bachelor’s degree in Computer Science or related field
At least 3 years of experience in backend development using Java
Proficiency in Java and other programming languages such as Python, Ruby, PHP, etc.
Knowledge of web development tools such as HTML, CSS, JavaScript, etc.
Experience with web services, APIs, databases, etc.
Familiarity with Git, CI/CD tools, cloud platforms, etc.
Good communication, collaboration, and problem-solving skills
If you are interested in this position, please apply with your resume and portfolio. We offer a competitive salary, benefits, and a dynamic work environment. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Senior Team Lead".to_string(),
description: r#"Algeria Startdown. is a company that provides innovative solutions for various industries. We are looking for a Senior Team Lead to join our team and lead a group of talented professionals.

As a Senior Team Lead, you will be responsible for:

Creating an inspiring team environment with an open communication culture
Setting clear team goals and expectations
Delegating tasks and setting deadlines for your team members
Monitoring the overall performance of the team and providing feedback and coaching
Resolving any issues or conflicts within the team or with other stakeholders
Supporting the preparation and delivery of status presentations and reports
Utilizing Lean Six Sigma tools, techniques, and methods to improve processes and quality
To be successful in this role, you should have:

Bachelor’s degree in a relevant field
At least 5 years of experience in a leadership or management role
Proven track record of leading and motivating teams to achieve goals
Excellent communication, collaboration, and problem-solving skills
Knowledge of Lean Six Sigma principles and practices
Ability to work under pressure and handle multiple tasks
If you are interested in this position, please apply with your resume and cover letter. We offer a competitive salary, benefits, and a supportive work environment. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Senior DevOps Engineer".to_string(),
            description: r#"Startupstare is looking for a Senior DevOps Engineer to join our team and help us deliver high-quality software solutions. You will be responsible for managing and improving communication between the operational and developmental sides of the software development process1, overseeing teams of junior software developers2, and ensuring the seamless deployment of software2.

As a Senior DevOps Engineer, you will:

Guide teams in designing, building, testing, and deploying changes to existing software1
Enhance the company’s IT infrastructure security protocols1
Identify manual processes that can be automated1
Conduct quality assurance to ensure that the software meets prescribed guidelines1
Roll out fixes and upgrades to software1
Secure software to prevent security breaches and other vulnerabilities13
To be successful in this role, you should have:

Bachelor’s degree in software engineering, computer science, information technology, information systems, or similar2
Master’s degree in a related field preferred2
Extensive experience in DevOps engineering, team management, and collaboration24
Proficiency in at least one scripting language3
Hands-on experience with databases including MySQL, Mongo & Elasticsearch3
Knowledge of Linux based infrastructure3
If you are interested in this position, please apply with your resume and cover letter. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Senior Frontend Engineer".to_string(),
            description: r#"Issab Refraf is looking for a Senior Frontend Engineer to join our team and help us create user-friendly and functional websites and applications. You will be responsible for building the user interface of our products, creating visual elements, and scripting interactions between users and the site12.

As a Senior Frontend Engineer, you will:

Review designs created by designers to ensure they are functional and meet usability standards1
Create prototypes and mockups of application screens and user interfaces in collaboration with designers and programmers1
Participate in code reviews and testing new features to ensure they are compatible with existing features1
Implement the user interface and engineer the experience of every site/software being put out by the business2
Work with a small and efficient team of engineers and product to deliver new changes often3
To be successful in this role, you should have:

Proven experience as a frontend developer or similar role3
Proficiency in HTML, CSS, JavaScript, and other web development tools3
Knowledge of frontend frameworks such as React, Angular, or Vue3
Familiarity with backend development and RESTful APIs3
Ability to work independently and collaboratively in a remote environment4
If you are interested in this position, please apply with your resume and portfolio. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Senior Backend Engineer".to_string(),
            description: r#"Alia Haddada is looking for a Senior Backend Engineer to join our team and help us build scalable and reliable web applications. You will be responsible for the server side code and APIs of our products, as well as the design, development, deployment and maintenance of new and existing features1.

As a Senior Backend Engineer, you will:

Design, develop, deploy and maintain web applications2
Manage the product lifecycle from concept through launch2
Understand the system designs and correct them where needed2
Design the code, develop the tests2
CI/CD the solution into existence as AWS Lambdas behind an API Gateway2
Work as a SME within a BI/ETL Engineering team to architect, design, develop, and maintain database driven .Net applications2
Monitor and recommend required upgrades and enhancements to existing systems2
Advocate for improvements to product quality, security, and performance that have particular impact across your team3
Solve technical problems of high scope and complexity3
To be successful in this role, you should have:

Proven experience as a backend developer or similar role1
Proficiency in .Net, C#, SQL Server, AWS Lambda, API Gateway, and other web development tools24
Knowledge of backend frameworks such as ASP.NET Core, Node.js, or Django1
Familiarity with frontend development and RESTful APIs1
Ability to work independently and collaboratively in a remote environment4
If you are interested in this position, please apply with your resume and portfolio. We look forward to hearing from you!"#.to_string(),
        },
        PostSeed {
            title: "Sales Person".to_string(),
            description: r#"Fatima Zeroual is a successful entrepreneur who runs a online clothing store. She is looking for a sales person who can help her grow her customer base and increase her revenue.

As a sales person, you will be responsible for:

- Contacting potential customers via phone, email, or social media and introducing them to Fatima's products
- Building rapport and trust with customers and providing them with personalized recommendations based on their preferences and needs
- Closing sales and processing orders using Fatima's e-commerce platform
- Following up with customers to ensure their satisfaction and encourage repeat purchases
- Providing feedback and suggestions to Fatima on how to improve her products and services

To be a successful sales person, you should have:

- Excellent communication and interpersonal skills
- A passion for fashion and a knowledge of current trends
- A self-motivated and goal-oriented attitude
- A willingness to learn and adapt to new challenges
- A reliable internet connection and a smartphone or computer

This is a commission-based position, meaning you will earn a percentage of every sale you make. You will also have flexible working hours and the opportunity to work from anywhere. If you are interested in joining Fatima's team, please send your resume and a cover letter."#.to_string(),
        },
        PostSeed {
            title: "Recruiter".to_string(),
            description: r#"Individual Mourat Weld El Ailma is looking for a Recruiter to join their team and help them find the best talent for their projects. The Recruiter will be responsible for sourcing, screening, interviewing and hiring candidates for various roles, such as developers, designers, marketers and more. The ideal candidate will have:

- Experience in recruitment, preferably in the IT sector
- Excellent communication and interpersonal skills
- Ability to work independently and collaboratively
- Knowledge of various recruitment tools and platforms
- Passion for finding and connecting with talented people

If you are interested in this position, please send your resume and cover letter"#.to_string(),
        },
        PostSeed {
            title: "Software Engineer".to_string(),
            description: r#"Are you a passionate and skilled software engineer who wants to work on exciting projects for various clients? If so, you might be the perfect fit for Individual Nadiatora Ramdani's team!

As a software engineer, you will:

- Design, code, test and deploy software applications using technologies like Java, Python, C#, HTML, CSS and more
- Collaborate with other developers and stakeholders to deliver high-quality products
- Learn new skills and tools to improve your performance and creativity
- Work independently and manage your own time and tasks

To apply for this position, you need:

- A bachelor's degree in computer science or a related field
- Experience in software development, preferably in the web or mobile domain
- Proficiency in one or more programming languages and frameworks
- Creativity and problem-solving skills

If this sounds like you, don't hesitate to send your resume and portfolio"#.to_string(),
        },
        PostSeed {
            title: "Product Manager".to_string(),
            description: r#"Company Yassiron is looking for a Product Manager to join their team and help them create and launch amazing products for their customers. The Product Manager will be responsible for defining the product vision, strategy and roadmap, as well as collaborating with engineers, designers, marketers and other stakeholders to deliver high-quality products that solve customer problems and meet business goals. The ideal candidate will have:

- A bachelor's degree in business, engineering or a related field
- Experience in product management, preferably in the tech industry
- Excellent communication and leadership skills
- Ability to work independently and cross-functionally
- Customer-centric and data-driven mindset

If you are interested in this position, please apply"#.to_string(),
        },
        PostSeed {
            title: "Data Scientist".to_string(),
            description: r#"Company Specific Emballage is looking for a Data Scientist to join their team and help them leverage data to optimize their packaging solutions and processes. The Data Scientist will be responsible for collecting, analyzing and interpreting data from various sources, such as customer feedback, market trends, production performance and more. The ideal candidate will have:

- A master's degree or PhD in statistics, computer science or a related field
- Experience in data science, preferably in the manufacturing or packaging industry
- Proficiency in one or more programming languages and tools, such as Python, R, SQL, Tableau and more
- Ability to work independently and collaboratively
- Analytical and problem-solving skills

If you are interested in this position, please send your resume and cover letter"#.to_string(),
        },
        PostSeed {
            title: "General Medicine Doctor".to_string(),
            description: r#"Are you a qualified and experienced general medicine doctor who wants to work for a company that values your skills and expertise? If so, you might be the perfect fit for Company Startdown.dz!

As a general medicine doctor, you will:

- Diagnose, treat and prevent various diseases and disorders
- Prescribe medications, order tests and refer patients to specialists when needed
- Provide quality health care services to patients
- Collaborate with other doctors and health care professionals
- Keep up to date with medical knowledge and best practices

To apply for this position, you need:

- A medical degree and a valid license to practice medicine in Algeria
- Experience in general medicine, preferably in a primary care setting
- Excellent communication and interpersonal skills
- Compassion and empathy for patients
- Ability to work independently and collaboratively

If this sounds like you, don't hesitate to contact us through this website"#.to_string(),
        },
        PostSeed {
            title: "General Surgery Doctor".to_string(),
            description: r#"Algeria Startdown Test is a leading healthcare provider in Algeria, offering innovative and high-quality medical services to patients across the country. We are looking for a General Surgery Doctor to join our team and perform a variety of surgical procedures for our patients.

As a General Surgery Doctor, you will be responsible for diagnosing and treating various diseases and disorders that require surgical intervention. You will perform operations involving the abdominal contents, the endocrine system, the skin, the breast, the soft tissue, and the vascular system. You will also examine patients, order tests, prescribe treatments, and follow up with them after surgery. You will work closely with other medical professionals and staff to ensure optimal patient care and safety.

You will need to have a degree in medicine (essential), certification through the Algerian Board of Medical Specialties (essential), 5 years of residency training in General Surgery (essential), current state medical license (essential), and completed Algerian Medical Licensing Examination (essential). You will also need to have 3 years of experience practicing as a General Surgeon, fantastic manual dexterity, hand-eye coordination, and visuospatial awareness, superb organizational and time management skills, excellent decision-making skills and communication, and a high level of empathy and compassion for patients.

If you are interested in this position, please apply online at algeriastartupjobs.com or send your resume and cover letter. We look forward to hearing from you soon."#.to_string(),
        },
        PostSeed {
            title: "Dentist".to_string(),
            description: r#"Are you a qualified dentist who can provide a range of dental services to our patients? If so, we would love to hear from you. Startupstare is a company that connects dental professionals with clients who need dental care. We are looking for a dentist to join our team and work on various projects across Algeria. You will be responsible for:

Meeting with clients to discuss and treat their dental concerns, perform regular cleanings and other preventive procedures, and establish a plan for better dental hygiene.
Performing dental procedures, such as extractions, root canals, filling cavities, correcting bite issues, applying prosthetics, sealants, whiteners, etc.
Prescribing medications for dental problems, such as painkillers or antibiotics.
Giving clients sedatives or anesthesia prior to administering treatments.
Ordering diagnostic measures, such as x-rays, models, etc.
Using tools, such as drills, probes, brushes, or mirrors, to examine and treat teeth and mouth.
Keeping records relating to the oral health of clients and the treatments given to them.
Managing and communicating with other staff members to provide quality dental care.
To be a successful candidate, you should have:

A doctorate of dental surgery or dental medicine.
A state license and malpractice insurance.
Strong computer skills and experience with healthcare databases and applications.
Willingness to comply with all local, state, and federal laws regarding dental and health care.
Excellent written and verbal communication skills, the ability to keep detailed records.
Comprehensive knowledge of dental procedures, tools, and diagnostics.
Good management skills.
If you are interested in this opportunity, please send your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Nurse".to_string(),
            description: r#"Individual Issab Refraf is looking for a Nurse to provide medical and nursing care to a family member at home. The Nurse will be responsible for administering treatment, performing exams, monitoring vitals, and providing psychological support. The Nurse will also collaborate with other health care professionals and follow care regulations and standards.

The ideal candidate should have a diploma or degree in nursing, a valid license to practice, and proven nursing experience. The candidate should also have excellent communication, problem-solving, and teamwork skills. The candidate should be compassionate, caring, and respectful of the patient’s needs and preferences.

The Nurse will work on a flexible schedule and will be compensated based on experience and qualifications. The Nurse will also receive training and support from the employer.

If you are interested in this position, please send your resume and cover letter"#.to_string(),
        },
        PostSeed {
            title: "Pharmacist".to_string(),
            description: r#"Here is a possible job description without headlines for the short description you provided:

Alia Haddada is looking for a Pharmacist to join her team at algeriastartupjobs.com. As a Pharmacist, you will be responsible for preparing and dispensing medications to customers according to their prescriptions or needs. You will also provide information and advice on how to use the medications safely and effectively.

To be successful in this role, you should have a degree in pharmacy, a valid license to practice, and experience in a retail or clinical setting. You should also have excellent communication and customer service skills, as well as knowledge of pharmacy software and legal regulations. You should be able to work independently and as part of a team, and handle multiple tasks in a fast-paced environment.

If you are interested in this opportunity, please send your resume and cover letter to alia.haddada@algeriastartupjobs.com. We look forward to hearing from you."#.to_string(),
        },
        PostSeed {
            title: "Physiotherapist".to_string(),
            description: r#"Are you a qualified physiotherapist who is passionate about helping people improve their physical mobility and wellbeing? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for a physiotherapist to join our team and work with our client, Fatima Zeroual, who is looking for a physiotherapist to assist her with her recovery from a recent injury. You will be responsible for assessing her physical condition, designing and implementing a personalized treatment plan, and providing education and advice on how to prevent further injury and maintain a healthy lifestyle.

To be successful in this role, you should have a degree in physiotherapy, experience working as a physiotherapist, and good interpersonal skills. You should also have the ability to work independently and as part of a team, and have good administration skills. You should be familiar with various physiotherapy techniques, such as manual therapy, therapeutic exercise, and electrotherapy.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Fatima via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Psychologist".to_string(),
            description: r#"Are you a qualified psychologist who is passionate about helping people improve their mental health and wellbeing? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for a psychologist to join our team and work with our client, Mourat Weld El Ailma, who is looking for a psychologist to assist him with his emotional and behavioral issues. You will be responsible for assessing his mental condition, diagnosing his problems, and providing him with individualized treatment options. You will also provide education and advice on how to cope with his challenges and prevent further difficulties.

To be successful in this role, you should have a doctoral degree in clinical or counseling psychology, a license to practice in your jurisdiction, and experience working as a psychologist or in a related field. You should also have excellent interpersonal and communication skills, and the ability to empathize with a wide range of people. You should be familiar with various psychological theories and practices, such as psychotherapy, cognitive-behavioral therapy, and diagnostic tests.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Mourat via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Veterinarian".to_string(),
            description: r#"Are you a qualified veterinarian who is passionate about providing healthcare services to animals? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for a veterinarian to join our team and work with our client, Nadiatora Ramdani, who is looking for a veterinarian to take care of her pets. You will be responsible for examining her animals and checking their health status, diagnosing and treating their illnesses and injuries, performing surgeries and vaccinations, and prescribing medication. You will also provide education and advice on nutrition, preventive healthcare, and general care.

To be successful in this role, you should have a Doctor of Veterinary Medicine (DVM) degree, a license to practice in your jurisdiction, and experience working as a veterinarian or in a related field. You should also have excellent communication skills, problem-solving skills, and compassion for animals and their owners. You should be familiar with various medical equipment and anesthesia procedures.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Nadiatora via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Accountant".to_string(),
            description: r#"Are you a qualified accountant who is passionate about managing financial transactions and preparing financial reports? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for an accountant to join our team and work with our client, Yassiron, a company that provides innovative solutions for various industries. You will be responsible for complying with all accounting and financial regulations, compiling, analyzing, and reporting financial data, creating periodic reports, such as balance sheets, profit and loss statements, etc., presenting data to managers, investors, and other entities, maintaining accurate financial records, performing audits and resolving discrepancies, computing taxes and preparing tax returns.

To be successful in this role, you should have a bachelor’s degree in accounting, finance, or a related field, or an equivalent combination of education, training and experience. You should also have 2-5 years of accounting/finance experience, excellent knowledge of accounting regulations and procedures, including the Generally Accepted Accounting Principles (GAAP), hands-on experience with accounting software like FreshBooks and QuickBooks, advanced MS Excel skills including Vlookups and pivot tables, strong analytical and problem-solving skills, excellent interpersonal and communication skills.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Yassiron via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Lawyer".to_string(),
            description: r#"Are you a qualified lawyer who is passionate about advising and representing clients on legal matters? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for a lawyer to join our team and work with our client, Specific Emballage, a company that provides packaging solutions for various industries. You will be responsible for interpreting laws, rulings and regulations for the company, conducting legal research and gathering evidence, drafting, reviewing and managing contracts and deeds, ensuring compliance with legal policies and regulations, offering legal representation at arbitration or mediation hearings, and preparing pleadings, notices and making appearances in court.

To be successful in this role, you should have a bachelor’s degree in law, be an admitted attorney of at least two years standing, and have experience in drafting, negotiating and reviewing legal documents. You should also have analytical thinking skills, sound judgment and decision-making skills, and excellent communication and public speaking skills. You should be familiar with various legal concepts and practices, such as contract law, corporate law, intellectual property law, etc.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Specific Emballage via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "English Teacher".to_string(),
            description: r#"Are you a qualified English teacher who is passionate about teaching reading, writing, and speaking skills to students? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for an English teacher to join our team and work with our client, Startdown.dz, a company that provides online courses and coaching for entrepreneurs. You will be responsible for teaching basic English skills to students from elementary school through to high school, using different methods and styles of teaching depending on the content and level. You will also compose lesson plans, engage students in meaningful and fun activities, instruct students about the structure and content of the English language, and assess their progress.

To be successful in this role, you should have a bachelor’s degree in English and education, and at least 5 years of experience in a teaching role. You should also be a first language English speaker, with excellent communication and interpersonal skills. You should have the ability to compile lesson plans efficiently, execute lessons with meaning, and create an environment conducive to learning.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Startdown.dz via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Math Teacher".to_string(),
            description: r#"Are you a qualified math teacher who is passionate about teaching mathematics and related subjects to students? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for a math teacher to join our team and work with our client, Algeria Startdown Test, a company that provides online courses and coaching for entrepreneurs. You will be responsible for planning and presenting math lessons to facilitate students’ understanding and application of mathematical concepts, such as algebra, geometry, or calculus. You will also prepare and distribute learning materials, such as notes, assignments, and quizzes, source the resources and supplies needed for lessons, grade assignments and quizzes in a timely manner, and document and report on students’ progress.

To be successful in this role, you should have a bachelor’s degree in education with a specialization in mathematics, or equivalent, and proven experience as a math teacher. You should also have a thorough understanding of best practices in teaching, excellent verbal and written communication skills, organized and flexible disposition, and outstanding interpersonal skills.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Algeria Startdown Test via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "Amazight Teacher".to_string(),
            description: r#"Are you a qualified Amazight teacher who is passionate about teaching Amazight language and culture to students? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for an Amazight teacher to join our team and work with our client, Startupstare, a company that provides online courses and coaching for entrepreneurs. You will be responsible for teaching Amazight language skills, such as reading, writing, and speaking, as well as Amazight culture and history, to students from elementary school through to high school. You will also prepare and distribute learning materials, such as notes, assignments, and quizzes, source the resources and supplies needed for lessons, grade assignments and quizzes in a timely manner, and document and report on students’ progress.

To be successful in this role, you should have a bachelor’s degree in education with a specialization in Amazight language and culture, or equivalent, and proven experience as an Amazight teacher. You should also have a thorough understanding of best practices in teaching, excellent communication and interpersonal skills, organized and flexible disposition, and outstanding cultural sensitivity skills.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Startupstare via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
        PostSeed {
            title: "French Teacher".to_string(),
            description: r#"Are you a qualified French teacher who is passionate about teaching French language and culture to students? If so, we have an exciting opportunity for you at algeriastartupjobs.com!

We are looking for a French teacher to join our team and work with our client, Issab Refraf, who is looking for a French teacher to help him improve his French skills. You will be responsible for teaching French language skills, such as reading, writing, and speaking, as well as French culture and history, to Issab Refraf. You will also prepare and distribute learning materials, such as notes, assignments, and quizzes, source the resources and supplies needed for lessons, grade assignments and quizzes in a timely manner, and document and report on Issab Refraf’s progress.

To be successful in this role, you should have a bachelor’s degree or equivalent in French language or literature, and proven experience as a French teacher. You should also have a thorough knowledge of the French language and culture, excellent communication and interpersonal skills, organized and flexible disposition, and outstanding cultural sensitivity skills.

This is a part-time, contract-based position that offers flexible hours and competitive pay. You will be working remotely from your home or office, and communicating with Issab Refraf via phone, email, or video call. You will also have access to our online platform where you can manage your schedule, track your progress, and receive feedback.

If you are interested in this opportunity, please apply online with your resume and cover letter. We look forward to hearing from you soon!"#.to_string(),
        },
    ];

  let non_admin_accounts = generate_accounts_seed()
    .into_iter()
    .filter(|account| match account.r#type {
      AccountType::Individual { .. } => true,
      AccountType::Company { .. } => true,
      _ => false,
    })
    .collect::<Vec<Account>>();
  let non_admin_accounts_len = non_admin_accounts.len();
  let categories = generate_categories_seed();
  let tags = generate_tags_seed();
  let total_posts_len = jobs.len();

  generate_many_post_mocks_with_overwrite(
    0,
    total_posts_len as i32,
    Some(|id| {
      let post = &jobs[id as usize];
      let title = titlecase(&post.title);
      let poster = &non_admin_accounts[id as usize % non_admin_accounts_len];
      let category = &categories[id as usize % categories.len()];
      let short_description = format!("{} is looking for a {}", poster.get_display_name(), title);

      let tags_found_on_description = tags
        .iter()
        .filter(|tag| fuzzy_word_exists(&tag.name, &post.description, 0.5))
        .collect::<Vec<&Tag>>();

      PartialPost {
        id: None,
        slug: Some(
          format!("{}_{}", post.title, id)
            .to_string()
            .replace("+", "_plus")
            .replace(" ", "_"),
        ),
        title: Some(title.clone()),
        poster_id: Some(poster.id),
        short_description: Some(short_description),
        description: Some(post.description.clone()),
        category_id: Some(category.id),
        tag_ids: Some(tags_found_on_description.iter().map(|tag| tag.id).collect()),
      }
    }),
  )
}
