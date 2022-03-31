ALTER TABLE accounts_payables ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
/*ALTER TABLE api_tokens ADD FOREIGN KEY (company_id) REFERENCES companys(id) ON
 * DELETE CASCADE ON UPDATE CASCADE;*/
ALTER TABLE applicant_interviews ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE applicant_reviewers ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE applicant_reviews ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE applicants ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE asset_items ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE auth_user_logins ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE auth_users ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE barcode_scans ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE bookings ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE buildings ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE certificates ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE conference_rooms ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE credit_card_transactions ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE expensed_items ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE functions ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE github_repos ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE groups ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE inbound_shipments ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE journal_club_meetings ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE journal_club_papers ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE links ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE mailing_list_subscribers ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE outbound_shipments ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE package_pickups ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE page_views ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE rack_line_subscribers ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE recorded_meetings ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE rfds ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE software_vendors ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE swag_inventory_items ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE swag_items ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE users ADD FOREIGN KEY (cio_company_id) REFERENCES companys(id) ON DELETE CASCADE ON UPDATE CASCADE;
