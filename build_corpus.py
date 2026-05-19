import re

def clean_and_build_corpus(pdf_path, output_txt):
    try:
        from pypdf import PdfReader
    except ImportError:
        print("Please install pypdf first: pip install pypdf")
        return

    reader = PdfReader(pdf_path)
    full_text = []

    print(f"Processing {len(reader.pages)} pages...")
    for page in reader.pages:
        text = page.extract_text()
        if text:
            # Convert to lowercase and change newlines to spaces
            text = text.lower().replace('\n', ' ')
            # Remove page numbers, headers, and unwanted non-ascii characters
            text = re.sub(r'[^a-z0-9\s_\-\.\=\+\/\*]', '', text)
            full_text.append(text)

    # Join everything into a single massive stream
    combined_corpus = " ".join(full_text)
    # Collapse multiple spaces into a single space
    combined_corpus = re.sub(r'\s+', ' ', combined_corpus)

    with open(output_txt, 'w', encoding='utf-8') as f:
        f.write(combined_corpus.strip())
        
    print(f"Successfully compiled {len(combined_corpus.split())} tokens into {output_txt}!")

# Example usage: Drop an open-source physics PDF in your desktop folder
clean_and_build_corpus("university_physics.pdf", "corpus.txt")