import pdfplumber
import json

if __name__ == '__main__':
    import sys
    file_path = "src/docs/112年度營利事業各業擴大書審純益率、所得額及同業利潤標準.pdf"
    with pdfplumber.open(file_path) as pdf:
        all_tables = []
        for page in pdf.pages:
            tables = page.extract_tables()
            all_tables.extend(tables)
